use auth::role::{DataValue, Role};

use crate::tests::data::role_a;

use super::data::role_b;

#[test]
fn parse_roles() {
    let role_in = role_a();

    let raw = role_in.to_string_pretty().unwrap();
    let role_out: Role = serde_json::from_str(&raw).unwrap();

    assert_eq!(role_in, role_out);
}

#[test]
fn merge_roles() {
    let mut default_role = role_a();
    let local_role = role_b();

    local_role.items.merge(&mut default_role.items);

    let item_2 = default_role.items.find("item_2").unwrap();

    let sub_item_2 = item_2.items.find("sub_item_2").unwrap();

    assert_eq!(sub_item_2.values.inner()[0].name, "value_2");
    assert_eq!(sub_item_2.values.inner()[0].data, DataValue::Float(5.0));

    let sub_item_1 = item_2.items.find("sub_item_1").unwrap();

    assert_eq!(sub_item_1.values.inner()[0].name, "value_4");
    assert_eq!(sub_item_1.values.inner()[0].data, DataValue::Boolean(false));

    assert_eq!(sub_item_1.values.inner()[1].name, "value_3");
    assert_eq!(
        sub_item_1.values.inner()[1].data,
        DataValue::String("text".to_string())
    );

    let item_1 = default_role.items.find("item_1").unwrap();

    let sub_item_4 = item_1.items.find("sub_item_4").unwrap();

    assert_eq!(sub_item_4.values.inner()[0].name, "value_6");
    assert_eq!(sub_item_4.values.inner()[0].data, DataValue::Integer(-2));

    assert_eq!(sub_item_4.values.inner()[1].name, "value_3");
    assert_eq!(sub_item_4.values.inner()[1].data, DataValue::Integer(2));
}
