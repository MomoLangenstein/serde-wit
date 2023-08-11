wit_bindgen::generate!({ path: "../../ser", world: "ser", exports: {
    world: SerTest,
    "test:ser/test": SerTest,
} });

struct SerTest;

impl Ser for SerTest {
    fn run() -> Result<(), String> {
        let value = serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });
        
        println!("{value}");

        Err(String::from("Hello world!"))
    }
}

fn main() {}
