// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

//lterator tra 计仅要求实现一个方法：
//next ：
// - 每次返回迭代器中的一项
// - 返回结果包裹在 Some 里
// - 迭代结束，返回 None
// 可以在迭代器上直接调用 next 方法

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}
//在上述代码中，使用iter()方法创建迭代器时，迭代器会返回集合中元素的引用，而不是元素的所有权。
//因此，next()方法返回的是Some(&String)，而不是Some(String)。
