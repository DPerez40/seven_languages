const std = @import("std");
const expect = std.testing.expect;
const MyErrors = error{EmptyInput};

pub fn isPalindrome(word: []const u8) bool {
    var i: usize = 0;
    var j: usize = word.len - 1;

    while (i < j) {
        if (std.ascii.toLower(word[i]) != std.ascii.toLower(word[j])) {
            return false;
        }
        i += 1;
        j -= 1;
    }

    return true;
}

pub fn reverseWord(allocator: std.mem.Allocator, word: []const u8) ![]u8 {
    if (word.len == 0) {
        return MyErrors.EmptyInput;
    }

    var reversed = try allocator.alloc(u8, word.len);
    var i: usize = 0;
    while (i < word.len) : (i += 1) {
        reversed[i] = word[word.len - 1 - i];
    }
    return reversed;
}

pub fn trim_newlines(s: []u8) []u8 {
    var end = s.len;
    while (end > 0 and (s[end - 1] == '\n' or s[end - 1] == '\r')) {
        end -= 1;
    }
    return s[0..end];
}

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();

    try stdout.print("Enter a word: ", .{});
    var buffer: [100]u8 = undefined;
    const line = try stdin.readUntilDelimiterOrEof(&buffer, '\n') orelse {
        try stdout.print("No input received.\n", .{});
        return;
    };
    const word = trim_newlines(line);

    if (word.len == 0) {
        try stdout.print("Empty word entered.\n", .{});
        return;
    }

    if(isPalindrome(word)) {
        try stdout.print("Yep, '{s}' is a palindrome!\n", .{word});
    } else {
        try stdout.print("Nope, '{s}' is not a palindrome!\n", .{word});
    }

    const reversed = try reverseWord(allocator, word);
    defer allocator.free(reversed);

    try stdout.print("Reversed: {s}\n", .{reversed});
}

test "Racecar is a palindrome." {
    try expect(isPalindrome("racecar"));
}

test "Hello is not a palindrome." {
    try expect(!isPalindrome("hello"));
}

test "Reverse word works." {
    const allocator = std.testing.allocator;
    const result = try reverseWord(allocator, "hello");
    defer allocator.free(result);
    try expect(std.mem.eql(u8, result, "olleh"));
}

test "Reverse empty word returns error." {
    const allocator = std.testing.allocator;
    try std.testing.expectError(MyErrors.EmptyInput, reverseWord(allocator, ""));
}
