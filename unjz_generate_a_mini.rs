use std::collections::HashMap;

pub struct UnjzGenerator {
    device_name: String,
    components: Vec<String>,
    dependencies: HashMap<String, String>,
}

impl UnjzGenerator {
    pub fn new(device_name: String) -> Self {
        UnjzGenerator {
            device_name,
            components: vec![],
            dependencies: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: String) {
        self.components.push(component);
    }

    pub fn add_dependency(&mut self, component: String, dependency: String) {
        self.dependencies.insert(component, dependency);
    }

    pub fn generate_code(&self) -> String {
        let mut code = format!("// {} IoT Device\n", self.device_name);
        code.push_str("use esp_idf_sys as _;\n");
        code.push_str("use esp_idf_svc::{wifi, gpio};//\n");
        code.push_str("\nfn main() {\n");

        for component in &self.components {
            code.push_str(&format!("    let {} = {{ /* initialization */ }};\n", component));
        }

        for (component, dependency) in &self.dependencies {
            code.push_str(&format!("    {}::init({});\n", component, dependency));
        }

        code.push_str("}\n");
        code
    }
}

fn main() {
    let mut generator = UnjzGenerator::new("MyIoTDevice".to_string());
    generator.add_component("wifi".to_string());
    generator.add_component("gpio".to_string());
    generator.add_dependency("wifi".to_string(), "WIFI_SSID".to_string());
    generator.add_dependency("gpio".to_string(), "GPIO_PIN".to_string());

    let code = generator.generate_code();
    println!("{}", code);
}