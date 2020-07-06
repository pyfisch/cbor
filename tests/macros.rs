#![cfg(feature = "std")]

use serde_cbor::{cbor, value::to_value, Value, Value::Null};

macro_rules! exec {
    ($answer:expr, $($question:tt)+) => {
        assert_eq!($answer, cbor!($($question)+).unwrap())
    };
}

macro_rules! map {
    ($($key:expr => $val:expr),* $(,)*) => {
        {
            #[allow(unused_mut)]
            let mut map = ::std::collections::BTreeMap::<Value, Value>::new();

            $(
                map.insert(
                    to_value($key).unwrap(),
                    to_value($val).unwrap()
                );
            )*

            Value::Map(map)
        }
    };
}

macro_rules! arr {
    ($($val:expr),*) => {
        Value::Array(vec![$(
            to_value($val).unwrap()
        ),*])
    };
}

#[test]
fn simple() {
    let bytes = serde_bytes::Bytes::new(b"\x00\x01\x02");

    exec!(Value::Null, null);
    exec!(Value::Bool(true), true);
    exec!(Value::Bool(false), false);
    exec!(Value::Text("foo".into()), "foo");
    exec!(Value::Bytes(vec![0, 1, 2]), bytes);

    exec!(Value::Integer(123), 123);
    exec!(Value::Integer(-123), -123);
    exec!(Value::Float(1.23), 1.23);
    exec!(Value::Float(-1.23), -1.23);
    exec!(Value::Float(2.5e+1), 2.5e+1);
    exec!(Value::Float(-2.5e+1), -2.5e+1);
}

#[test]
fn array() {
    exec!(arr![], []);

    exec!(arr![Null], [null]);
    exec!(arr![true], [true]);
    exec!(arr![false], [false]);
    exec!(arr!["foo"], ["foo"]);
    exec!(arr![123], [123]);
    exec!(arr![-123], [-123]);
    exec!(arr![1.23], [1.23]);
    exec!(arr![-1.23], [-1.23]);
    exec!(arr![2.5e+1], [2.5e+1]);
    exec!(arr![2.5e+1], [2.5e+1]);
    exec!(arr![[1, 2]], [[1, 2]]);
    exec!(arr![map! {1=>2,3=>4}], [{1=>2,3=>4}]);

    exec!(arr![Null, Null], [null, null]);
    exec!(arr![Null, true], [null, true]);
    exec!(arr![Null, false], [null, false]);
    exec!(arr![Null, "foo"], [null, "foo"]);
    exec!(arr![Null, 123], [null, 123]);
    exec!(arr![Null, -123], [null, -123]);
    exec!(arr![Null, 1.23], [null, 1.23]);
    exec!(arr![Null, -1.23], [null, -1.23]);
    exec!(arr![Null, 2.5e+1], [null, 2.5e+1]);
    exec!(arr![Null, 2.5e+1], [null, 2.5e+1]);
    exec!(arr![Null, [1, 2]], [null, [1, 2]]);
    exec!(arr![Null, map! {1=>2,3=>4}], [null, {1=>2,3=>4}]);
    exec!(arr![true, Null], [true, null]);
    exec!(arr![true, true], [true, true]);
    exec!(arr![true, false], [true, false]);
    exec!(arr![true, "foo"], [true, "foo"]);
    exec!(arr![true, 123], [true, 123]);
    exec!(arr![true, -123], [true, -123]);
    exec!(arr![true, 1.23], [true, 1.23]);
    exec!(arr![true, -1.23], [true, -1.23]);
    exec!(arr![true, 2.5e+1], [true, 2.5e+1]);
    exec!(arr![true, 2.5e+1], [true, 2.5e+1]);
    exec!(arr![true, [1, 2]], [true, [1, 2]]);
    exec!(arr![true, map! {1=>2,3=>4}], [true, {1=>2,3=>4}]);
    exec!(arr![false, Null], [false, null]);
    exec!(arr![false, true], [false, true]);
    exec!(arr![false, false], [false, false]);
    exec!(arr![false, "foo"], [false, "foo"]);
    exec!(arr![false, 123], [false, 123]);
    exec!(arr![false, -123], [false, -123]);
    exec!(arr![false, 1.23], [false, 1.23]);
    exec!(arr![false, -1.23], [false, -1.23]);
    exec!(arr![false, 2.5e+1], [false, 2.5e+1]);
    exec!(arr![false, 2.5e+1], [false, 2.5e+1]);
    exec!(arr![false, [1, 2]], [false, [1, 2]]);
    exec!(arr![false, map! {1=>2,3=>4}], [false, {1=>2,3=>4}]);
    exec!(arr!["foo", Null], ["foo", null]);
    exec!(arr!["foo", true], ["foo", true]);
    exec!(arr!["foo", false], ["foo", false]);
    exec!(arr!["foo", "foo"], ["foo", "foo"]);
    exec!(arr!["foo", 123], ["foo", 123]);
    exec!(arr!["foo", -123], ["foo", -123]);
    exec!(arr!["foo", 1.23], ["foo", 1.23]);
    exec!(arr!["foo", -1.23], ["foo", -1.23]);
    exec!(arr!["foo", 2.5e+1], ["foo", 2.5e+1]);
    exec!(arr!["foo", 2.5e+1], ["foo", 2.5e+1]);
    exec!(arr!["foo", [1, 2]], ["foo", [1, 2]]);
    exec!(arr!["foo", map! {1=>2,3=>4}], ["foo", {1=>2,3=>4}]);
    exec!(arr![123, Null], [123, null]);
    exec!(arr![123, true], [123, true]);
    exec!(arr![123, false], [123, false]);
    exec!(arr![123, "foo"], [123, "foo"]);
    exec!(arr![123, 123], [123, 123]);
    exec!(arr![123, -123], [123, -123]);
    exec!(arr![123, 1.23], [123, 1.23]);
    exec!(arr![123, -1.23], [123, -1.23]);
    exec!(arr![123, 2.5e+1], [123, 2.5e+1]);
    exec!(arr![123, 2.5e+1], [123, 2.5e+1]);
    exec!(arr![123, [1, 2]], [123, [1, 2]]);
    exec!(arr![123, map! {1=>2,3=>4}], [123, {1=>2,3=>4}]);
    exec!(arr![-123, Null], [-123, null]);
    exec!(arr![-123, true], [-123, true]);
    exec!(arr![-123, false], [-123, false]);
    exec!(arr![-123, "foo"], [-123, "foo"]);
    exec!(arr![-123, 123], [-123, 123]);
    exec!(arr![-123, -123], [-123, -123]);
    exec!(arr![-123, 1.23], [-123, 1.23]);
    exec!(arr![-123, -1.23], [-123, -1.23]);
    exec!(arr![-123, 2.5e+1], [-123, 2.5e+1]);
    exec!(arr![-123, 2.5e+1], [-123, 2.5e+1]);
    exec!(arr![-123, [1, 2]], [-123, [1, 2]]);
    exec!(arr![-123, map! {1=>2,3=>4}], [-123, {1=>2,3=>4}]);
    exec!(arr![1.23, Null], [1.23, null]);
    exec!(arr![1.23, true], [1.23, true]);
    exec!(arr![1.23, false], [1.23, false]);
    exec!(arr![1.23, "foo"], [1.23, "foo"]);
    exec!(arr![1.23, 123], [1.23, 123]);
    exec!(arr![1.23, -123], [1.23, -123]);
    exec!(arr![1.23, 1.23], [1.23, 1.23]);
    exec!(arr![1.23, -1.23], [1.23, -1.23]);
    exec!(arr![1.23, 2.5e+1], [1.23, 2.5e+1]);
    exec!(arr![1.23, 2.5e+1], [1.23, 2.5e+1]);
    exec!(arr![1.23, [1, 2]], [1.23, [1, 2]]);
    exec!(arr![1.23, map! {1=>2,3=>4}], [1.23, {1=>2,3=>4}]);
    exec!(arr![-1.23, Null], [-1.23, null]);
    exec!(arr![-1.23, true], [-1.23, true]);
    exec!(arr![-1.23, false], [-1.23, false]);
    exec!(arr![-1.23, "foo"], [-1.23, "foo"]);
    exec!(arr![-1.23, 123], [-1.23, 123]);
    exec!(arr![-1.23, -123], [-1.23, -123]);
    exec!(arr![-1.23, 1.23], [-1.23, 1.23]);
    exec!(arr![-1.23, -1.23], [-1.23, -1.23]);
    exec!(arr![-1.23, 2.5e+1], [-1.23, 2.5e+1]);
    exec!(arr![-1.23, 2.5e+1], [-1.23, 2.5e+1]);
    exec!(arr![-1.23, [1, 2]], [-1.23, [1, 2]]);
    exec!(arr![-1.23, map! {1=>2,3=>4}], [-1.23, {1=>2,3=>4}]);
    exec!(arr![2.5e+1, Null], [2.5e+1, null]);
    exec!(arr![2.5e+1, true], [2.5e+1, true]);
    exec!(arr![2.5e+1, false], [2.5e+1, false]);
    exec!(arr![2.5e+1, "foo"], [2.5e+1, "foo"]);
    exec!(arr![2.5e+1, 123], [2.5e+1, 123]);
    exec!(arr![2.5e+1, -123], [2.5e+1, -123]);
    exec!(arr![2.5e+1, 1.23], [2.5e+1, 1.23]);
    exec!(arr![2.5e+1, -1.23], [2.5e+1, -1.23]);
    exec!(arr![2.5e+1, 2.5e+1], [2.5e+1, 2.5e+1]);
    exec!(arr![2.5e+1, 2.5e+1], [2.5e+1, 2.5e+1]);
    exec!(arr![2.5e+1, [1, 2]], [2.5e+1, [1, 2]]);
    exec!(arr![2.5e+1, map! {1=>2,3=>4}], [2.5e+1, {1=>2,3=>4}]);
    exec!(arr![2.5e+1, Null], [2.5e+1, null]);
    exec!(arr![2.5e+1, true], [2.5e+1, true]);
    exec!(arr![2.5e+1, false], [2.5e+1, false]);
    exec!(arr![2.5e+1, "foo"], [2.5e+1, "foo"]);
    exec!(arr![2.5e+1, 123], [2.5e+1, 123]);
    exec!(arr![2.5e+1, -123], [2.5e+1, -123]);
    exec!(arr![2.5e+1, 1.23], [2.5e+1, 1.23]);
    exec!(arr![2.5e+1, -1.23], [2.5e+1, -1.23]);
    exec!(arr![2.5e+1, 2.5e+1], [2.5e+1, 2.5e+1]);
    exec!(arr![2.5e+1, 2.5e+1], [2.5e+1, 2.5e+1]);
    exec!(arr![2.5e+1, [1, 2]], [2.5e+1, [1, 2]]);
    exec!(arr![2.5e+1, map! {1=>2,3=>4}], [2.5e+1, {1=>2,3=>4}]);
    exec!(arr![[1, 2], Null], [[1, 2], null]);
    exec!(arr![[1, 2], true], [[1, 2], true]);
    exec!(arr![[1, 2], false], [[1, 2], false]);
    exec!(arr![[1, 2], "foo"], [[1, 2], "foo"]);
    exec!(arr![[1, 2], 123], [[1, 2], 123]);
    exec!(arr![[1, 2], -123], [[1, 2], -123]);
    exec!(arr![[1, 2], 1.23], [[1, 2], 1.23]);
    exec!(arr![[1, 2], -1.23], [[1, 2], -1.23]);
    exec!(arr![[1, 2], 2.5e+1], [[1, 2], 2.5e+1]);
    exec!(arr![[1, 2], 2.5e+1], [[1, 2], 2.5e+1]);
    exec!(arr![[1, 2], [1, 2]], [[1, 2], [1, 2]]);
    exec!(arr![[1, 2], map! {1=>2,3=>4}], [[1, 2], {1=>2,3=>4}]);
    exec!(arr![map! {1=>2,3=>4}, Null], [{1=>2,3=>4}, null]);
    exec!(arr![map! {1=>2,3=>4}, true], [{1=>2,3=>4}, true]);
    exec!(arr![map! {1=>2,3=>4}, false], [{1=>2,3=>4}, false]);
    exec!(arr![map! {1=>2,3=>4}, "foo"], [{1=>2,3=>4}, "foo"]);
    exec!(arr![map! {1=>2,3=>4}, 123], [{1=>2,3=>4}, 123]);
    exec!(arr![map! {1=>2,3=>4}, -123], [{1=>2,3=>4}, -123]);
    exec!(arr![map! {1=>2,3=>4}, 1.23], [{1=>2,3=>4}, 1.23]);
    exec!(arr![map! {1=>2,3=>4}, -1.23], [{1=>2,3=>4}, -1.23]);
    exec!(arr![map! {1=>2,3=>4}, 2.5e+1], [{1=>2,3=>4}, 2.5e+1]);
    exec!(arr![map! {1=>2,3=>4}, 2.5e+1], [{1=>2,3=>4}, 2.5e+1]);
    exec!(arr![map! {1=>2,3=>4}, [1, 2]], [{1=>2,3=>4}, [1, 2]]);
    exec!(arr![map! {1=>2,3=>4}, map! {1=>2,3=>4}], [{1=>2,3=>4}, {1=>2,3=>4}]);
}

#[test]
fn map() {
    exec!(map! {}, {});

    exec!(map! {Null => Null}, { null => null });
    exec!(map! {Null => true}, { null => true });
    exec!(map! {Null => false}, { null => false });
    exec!(map! {Null => "foo"}, { null => "foo" });
    exec!(map! {Null => 123}, { null => 123 });
    exec!(map! {Null => -123}, { null => -123 });
    exec!(map! {Null => 1.23}, { null => 1.23 });
    exec!(map! {Null => -1.23}, { null => -1.23 });
    exec!(map! {Null => 2.5e+1}, { null => 2.5e+1 });
    exec!(map! {Null => 2.5e+1}, { null => 2.5e+1 });
    exec!(map! {Null => [1, 2]}, { null => [1, 2] });
    exec!(map! {Null => map! {1=>2,3=>4}}, { null => {1=>2,3=>4} });
    exec!(map! {true => Null}, { true => null });
    exec!(map! {true => true}, { true => true });
    exec!(map! {true => false}, { true => false });
    exec!(map! {true => "foo"}, { true => "foo" });
    exec!(map! {true => 123}, { true => 123 });
    exec!(map! {true => -123}, { true => -123 });
    exec!(map! {true => 1.23}, { true => 1.23 });
    exec!(map! {true => -1.23}, { true => -1.23 });
    exec!(map! {true => 2.5e+1}, { true => 2.5e+1 });
    exec!(map! {true => 2.5e+1}, { true => 2.5e+1 });
    exec!(map! {true => [1, 2]}, { true => [1, 2] });
    exec!(map! {true => map! {1=>2,3=>4}}, { true => {1=>2,3=>4} });
    exec!(map! {false => Null}, { false => null });
    exec!(map! {false => true}, { false => true });
    exec!(map! {false => false}, { false => false });
    exec!(map! {false => "foo"}, { false => "foo" });
    exec!(map! {false => 123}, { false => 123 });
    exec!(map! {false => -123}, { false => -123 });
    exec!(map! {false => 1.23}, { false => 1.23 });
    exec!(map! {false => -1.23}, { false => -1.23 });
    exec!(map! {false => 2.5e+1}, { false => 2.5e+1 });
    exec!(map! {false => 2.5e+1}, { false => 2.5e+1 });
    exec!(map! {false => [1, 2]}, { false => [1, 2] });
    exec!(map! {false => map! {1=>2,3=>4}}, { false => {1=>2,3=>4} });
    exec!(map! {"foo" => Null}, { "foo" => null });
    exec!(map! {"foo" => true}, { "foo" => true });
    exec!(map! {"foo" => false}, { "foo" => false });
    exec!(map! {"foo" => "foo"}, { "foo" => "foo" });
    exec!(map! {"foo" => 123}, { "foo" => 123 });
    exec!(map! {"foo" => -123}, { "foo" => -123 });
    exec!(map! {"foo" => 1.23}, { "foo" => 1.23 });
    exec!(map! {"foo" => -1.23}, { "foo" => -1.23 });
    exec!(map! {"foo" => 2.5e+1}, { "foo" => 2.5e+1 });
    exec!(map! {"foo" => 2.5e+1}, { "foo" => 2.5e+1 });
    exec!(map! {"foo" => [1, 2]}, { "foo" => [1, 2] });
    exec!(map! {"foo" => map! {1=>2,3=>4}}, { "foo" => {1=>2,3=>4} });
    exec!(map! {123 => Null}, { 123 => null });
    exec!(map! {123 => true}, { 123 => true });
    exec!(map! {123 => false}, { 123 => false });
    exec!(map! {123 => "foo"}, { 123 => "foo" });
    exec!(map! {123 => 123}, { 123 => 123 });
    exec!(map! {123 => -123}, { 123 => -123 });
    exec!(map! {123 => 1.23}, { 123 => 1.23 });
    exec!(map! {123 => -1.23}, { 123 => -1.23 });
    exec!(map! {123 => 2.5e+1}, { 123 => 2.5e+1 });
    exec!(map! {123 => 2.5e+1}, { 123 => 2.5e+1 });
    exec!(map! {123 => [1, 2]}, { 123 => [1, 2] });
    exec!(map! {123 => map! {1=>2,3=>4}}, { 123 => {1=>2,3=>4} });
    exec!(map! {-123 => Null}, { -123 => null });
    exec!(map! {-123 => true}, { -123 => true });
    exec!(map! {-123 => false}, { -123 => false });
    exec!(map! {-123 => "foo"}, { -123 => "foo" });
    exec!(map! {-123 => 123}, { -123 => 123 });
    exec!(map! {-123 => -123}, { -123 => -123 });
    exec!(map! {-123 => 1.23}, { -123 => 1.23 });
    exec!(map! {-123 => -1.23}, { -123 => -1.23 });
    exec!(map! {-123 => 2.5e+1}, { -123 => 2.5e+1 });
    exec!(map! {-123 => 2.5e+1}, { -123 => 2.5e+1 });
    exec!(map! {-123 => [1, 2]}, { -123 => [1, 2] });
    exec!(map! {-123 => map! {1=>2,3=>4}}, { -123 => {1=>2,3=>4} });
    exec!(map! {1.23 => Null}, { 1.23 => null });
    exec!(map! {1.23 => true}, { 1.23 => true });
    exec!(map! {1.23 => false}, { 1.23 => false });
    exec!(map! {1.23 => "foo"}, { 1.23 => "foo" });
    exec!(map! {1.23 => 123}, { 1.23 => 123 });
    exec!(map! {1.23 => -123}, { 1.23 => -123 });
    exec!(map! {1.23 => 1.23}, { 1.23 => 1.23 });
    exec!(map! {1.23 => -1.23}, { 1.23 => -1.23 });
    exec!(map! {1.23 => 2.5e+1}, { 1.23 => 2.5e+1 });
    exec!(map! {1.23 => 2.5e+1}, { 1.23 => 2.5e+1 });
    exec!(map! {1.23 => [1, 2]}, { 1.23 => [1, 2] });
    exec!(map! {1.23 => map! {1=>2,3=>4}}, { 1.23 => {1=>2,3=>4} });
    exec!(map! {-1.23 => Null}, { -1.23 => null });
    exec!(map! {-1.23 => true}, { -1.23 => true });
    exec!(map! {-1.23 => false}, { -1.23 => false });
    exec!(map! {-1.23 => "foo"}, { -1.23 => "foo" });
    exec!(map! {-1.23 => 123}, { -1.23 => 123 });
    exec!(map! {-1.23 => -123}, { -1.23 => -123 });
    exec!(map! {-1.23 => 1.23}, { -1.23 => 1.23 });
    exec!(map! {-1.23 => -1.23}, { -1.23 => -1.23 });
    exec!(map! {-1.23 => 2.5e+1}, { -1.23 => 2.5e+1 });
    exec!(map! {-1.23 => 2.5e+1}, { -1.23 => 2.5e+1 });
    exec!(map! {-1.23 => [1, 2]}, { -1.23 => [1, 2] });
    exec!(map! {-1.23 => map! {1=>2,3=>4}}, { -1.23 => {1=>2,3=>4} });
    exec!(map! {2.5e+1 => Null}, { 2.5e+1 => null });
    exec!(map! {2.5e+1 => true}, { 2.5e+1 => true });
    exec!(map! {2.5e+1 => false}, { 2.5e+1 => false });
    exec!(map! {2.5e+1 => "foo"}, { 2.5e+1 => "foo" });
    exec!(map! {2.5e+1 => 123}, { 2.5e+1 => 123 });
    exec!(map! {2.5e+1 => -123}, { 2.5e+1 => -123 });
    exec!(map! {2.5e+1 => 1.23}, { 2.5e+1 => 1.23 });
    exec!(map! {2.5e+1 => -1.23}, { 2.5e+1 => -1.23 });
    exec!(map! {2.5e+1 => 2.5e+1}, { 2.5e+1 => 2.5e+1 });
    exec!(map! {2.5e+1 => 2.5e+1}, { 2.5e+1 => 2.5e+1 });
    exec!(map! {2.5e+1 => [1, 2]}, { 2.5e+1 => [1, 2] });
    exec!(map! {2.5e+1 => map! {1=>2,3=>4}}, { 2.5e+1 => {1=>2,3=>4} });
    exec!(map! {2.5e+1 => Null}, { 2.5e+1 => null });
    exec!(map! {2.5e+1 => true}, { 2.5e+1 => true });
    exec!(map! {2.5e+1 => false}, { 2.5e+1 => false });
    exec!(map! {2.5e+1 => "foo"}, { 2.5e+1 => "foo" });
    exec!(map! {2.5e+1 => 123}, { 2.5e+1 => 123 });
    exec!(map! {2.5e+1 => -123}, { 2.5e+1 => -123 });
    exec!(map! {2.5e+1 => 1.23}, { 2.5e+1 => 1.23 });
    exec!(map! {2.5e+1 => -1.23}, { 2.5e+1 => -1.23 });
    exec!(map! {2.5e+1 => 2.5e+1}, { 2.5e+1 => 2.5e+1 });
    exec!(map! {2.5e+1 => 2.5e+1}, { 2.5e+1 => 2.5e+1 });
    exec!(map! {2.5e+1 => [1, 2]}, { 2.5e+1 => [1, 2] });
    exec!(map! {2.5e+1 => map! {1=>2,3=>4}}, { 2.5e+1 => {1=>2,3=>4} });
    exec!(map! {[1, 2] => Null}, { [1, 2] => null });
    exec!(map! {[1, 2] => true}, { [1, 2] => true });
    exec!(map! {[1, 2] => false}, { [1, 2] => false });
    exec!(map! {[1, 2] => "foo"}, { [1, 2] => "foo" });
    exec!(map! {[1, 2] => 123}, { [1, 2] => 123 });
    exec!(map! {[1, 2] => -123}, { [1, 2] => -123 });
    exec!(map! {[1, 2] => 1.23}, { [1, 2] => 1.23 });
    exec!(map! {[1, 2] => -1.23}, { [1, 2] => -1.23 });
    exec!(map! {[1, 2] => 2.5e+1}, { [1, 2] => 2.5e+1 });
    exec!(map! {[1, 2] => 2.5e+1}, { [1, 2] => 2.5e+1 });
    exec!(map! {[1, 2] => [1, 2]}, { [1, 2] => [1, 2] });
    exec!(map! {[1, 2] => map! {1=>2,3=>4}}, { [1, 2] => {1=>2,3=>4} });
    exec!(map! {map! {1=>2,3=>4} => Null}, { {1=>2,3=>4} => null });
    exec!(map! {map! {1=>2,3=>4} => true}, { {1=>2,3=>4} => true });
    exec!(map! {map! {1=>2,3=>4} => false}, { {1=>2,3=>4} => false });
    exec!(map! {map! {1=>2,3=>4} => "foo"}, { {1=>2,3=>4} => "foo" });
    exec!(map! {map! {1=>2,3=>4} => 123}, { {1=>2,3=>4} => 123 });
    exec!(map! {map! {1=>2,3=>4} => -123}, { {1=>2,3=>4} => -123 });
    exec!(map! {map! {1=>2,3=>4} => 1.23}, { {1=>2,3=>4} => 1.23 });
    exec!(map! {map! {1=>2,3=>4} => -1.23}, { {1=>2,3=>4} => -1.23 });
    exec!(map! {map! {1=>2,3=>4} => 2.5e+1}, { {1=>2,3=>4} => 2.5e+1 });
    exec!(map! {map! {1=>2,3=>4} => 2.5e+1}, { {1=>2,3=>4} => 2.5e+1 });
    exec!(map! {map! {1=>2,3=>4} => [1, 2]}, { {1=>2,3=>4} => [1, 2] });
    exec!(map! {map! {1=>2,3=>4} => map! {1=>2,3=>4}}, { {1=>2,3=>4} => {1=>2,3=>4} });
}
