| Trường hợp                        | Kiểu nên dùng   |
| --------------------------------- | --------------- |
| Số nguyên thông thường            | `i32`           |
| Tuổi, điểm, byte RGB              | `u8`            |
| Cổng mạng (8080, 443...)          | `u16`           |
| Chỉ số mảng                       | `usize`         |
| Timestamp hoặc số rất lớn         | `i64` / `u64`   |
| Big Integer trong phạm vi 128 bit | `i128` / `u128` |

`use std::io::{self, Write}; // io là module phục vụ in/out `
`use std::fs; // fs là module làm việc với file`
`unwrap(); // unwrap để lấy giá trị, lỗi sẽ trả called`
`Result::unwrap() on an Err value: Os {`
`code: 2,`
` kind: NotFound,`
`message: "No such file or directory"`
`}`

- expect: value || "msg" err

* Ordering: enum biểu diễn phép so sánh
