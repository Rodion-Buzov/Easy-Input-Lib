En:
Hello everyone, I was studying the Rust programming language, and I encountered the problem of complex data entry from the keyboard when creating console programs, so I wrote a small library that simplifies this action.
P.S:
The idea is not entirely mine, I found one function on StackOverflow and made a library based on it.

Ru:
Всем привет, я изучал язык программировая Раст, и столкнулся с проблемой сложного ввода данных с клавиатуры при создании консольных программ, поэту я написал небольшую библеотеку, упрощающую это действие.
P.S:
Идея не совсем моя, я нашел на StackOverflow одну функцию и сделал на ее основе библиотеку.


Example:
fn main {
    let user_name:String = read_string("Enter your name: ");
    println!("Your name: {}", user_name);
}