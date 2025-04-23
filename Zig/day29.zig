const std = @import("std");

const MyErrors = error{
    DivideByZero,
    InvalidInput,
};

//Divide function with error handling
pub fn divide(a: f64, b: f64) !f64 {
    if (b == 0.0) {
        return MyErrors.DivideByZero;
    }
    return a / b;
}


//Trim user input of /n and /r
pub fn trim_newlines(s: []u8) []u8 {
    var end = s.len;

    while (end > 0 and (s[end - 1] == '\n' or s[end - 1] == '\r')) {
        end -= 1;
    }

    return s[0..end];
}

pub fn main() !void {
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();

    //First number
    try stdout.print("Enter first number: ", .{});
    var buf1: [100]u8 = undefined;
    const line1 = try stdin.readUntilDelimiterOrEof(&buf1, '\n') orelse {
        try stdout.print("Failed to read input.\n", .{});
        return;
    };
    const clean1 = trim_newlines(line1);
    const a = std.fmt.parseFloat(f64, clean1) catch {
        try stdout.print("Invalid input. Please enter a number.\n", .{});
        return;
    };

    //Second number
    try stdout.print("Enter second number: ", .{});
    var buf2: [100]u8 = undefined;
    const line2 = try stdin.readUntilDelimiterOrEof(&buf2, '\n') orelse {
        try stdout.print("Failed to read input.\n", .{});
        return;
    };
    const clean2 = trim_newlines(line2);
    const b = std.fmt.parseFloat(f64, clean2) catch {
        try stdout.print("Invalid input. Please enter a number.\n", .{});
        return;
    };

    //Division with error catch
    const result = divide(a, b) catch |err| {
        switch (err) {
            MyErrors.DivideByZero => {
                try stdout.print("Custom error: You tried to divide by zero and it is not possible.\n", .{});
            },
            MyErrors.InvalidInput => {
                try stdout.print("Custom error: One of your inputs was invalid.\n", .{});
            },
        }
        return;
    };

    try stdout.print("Result: {}\n", .{result});
}
