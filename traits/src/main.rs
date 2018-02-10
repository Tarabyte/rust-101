trait WithDefaultImpl {
    fn test(&self) -> String {
        "Hello".to_string()
    }
}

struct UsesDefault {
    name: String,
}

impl WithDefaultImpl for UsesDefault {

}

struct OverridesButUsesDefault {
    wrapped: UsesDefault,
}

impl WithDefaultImpl for OverridesButUsesDefault {
    fn test(&self) -> String {
        format!("{}, {}", self.wrapped.test(), self.wrapped.name)
    }
}

fn main() {
    let default = UsesDefault {
        name: "Basil".to_string()
    };

    println!("{}", &default.test());

    let wrapped = OverridesButUsesDefault {
        wrapped: default
    };

    println!("{}", &wrapped.test());
}
