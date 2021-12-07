use std::env;

fn main() {
    println!(r"Status: 200 OK
Content-Type: text/html
X-Powered-By: cgi_test

<html>
<head>
    <title>cgi_test</title>
</head>
<body>
    <h1>Hello from Rust!</h1>
    <h2>Arguments</h2>
    <ol>
");

    for arg in env::args() {
        println!("<li>{}</li>", arg);
    }

println!(r"
    </ol>

    <h2>Environment</h2>
    <table>
");

    for (key, value) in env::vars() {
        println!("<tr><td>{}</td><td>{}</td></tr>", key, value);
    }

    println!(r"</table>
</body></html>
");
}
