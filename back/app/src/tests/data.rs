use mongodb::bson::oid::ObjectId;

use userman_auth::roles::{DataOptions, DataValue, Item, Role, RoleItems, RoleValues, Value};

pub fn role_a() -> Role {
    Role {
        id: None,
        app: ObjectId::new(),
        name: "admin".to_string(),
        items: RoleItems::new(vec![
            Item {
                name: "item_2".to_string(),
                values: RoleValues::default(),
                items: RoleItems::new(vec![
                    Item {
                        name: "sub_item_2".to_string(),
                        values: RoleValues(vec![Value {
                            name: "value_2".to_string(),
                            data: DataValue::Float(2.5),
                            options: Some(DataOptions {
                                min_value: DataValue::Float(1.0),
                                max_value: DataValue::Float(100.0),
                            }),
                        }]),
                        items: RoleItems::default(),
                    },
                    Item {
                        name: "sub_item_1".to_string(),
                        values: RoleValues(vec![
                            Value {
                                name: "value_4".to_string(),
                                data: DataValue::Boolean(true),
                                options: None,
                            },
                            Value {
                                name: "value_3".to_string(),
                                data: DataValue::String("text".to_string()),
                                options: None,
                            },
                        ]),
                        items: RoleItems::default(),
                    },
                ]),
            },
            Item {
                name: "item_1".to_string(),
                values: RoleValues::default(),
                items: RoleItems::new(vec![Item {
                    name: "sub_item_4".to_string(),
                    values: RoleValues(vec![
                        Value {
                            name: "value_6".to_string(),
                            data: DataValue::Integer(-2),
                            options: None,
                        },
                        Value {
                            name: "value_3".to_string(),
                            data: DataValue::Integer(2),
                            options: None,
                        },
                    ]),
                    items: RoleItems::default(),
                }]),
            },
        ]),
        created_at: None,
        updated_at: None,
    }
}

pub fn role_b() -> Role {
    Role {
        id: None,
        app: ObjectId::new(),
        name: "admin".to_string(),
        items: RoleItems::new(vec![Item {
            name: "item_2".to_string(),
            values: RoleValues::default(),
            items: RoleItems::new(vec![
                Item {
                    name: "sub_item_2".to_string(),
                    values: RoleValues(vec![Value {
                        name: "value_2".to_string(),
                        data: DataValue::Float(5.0),
                        options: None,
                    }]),
                    items: RoleItems::default(),
                },
                Item {
                    name: "sub_item_1".to_string(),
                    values: RoleValues(vec![
                        Value {
                            name: "value_4".to_string(),
                            data: DataValue::Boolean(false),
                            options: None,
                        },
                        Value {
                            name: "value_3".to_string(),
                            data: DataValue::String("text".to_string()),
                            options: None,
                        },
                    ]),
                    items: RoleItems::default(),
                },
            ]),
        }]),
        created_at: None,
        updated_at: None,
    }
}
