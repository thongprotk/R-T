| Trường hợp                        | Kiểu nên dùng   |
| --------------------------------- | --------------- |
| Số nguyên thông thường            | `i32`           |
| Tuổi, điểm, byte RGB              | `u8`            |
| Cổng mạng (8080, 443...)          | `u16`           |
| Chỉ số mảng                       | `usize`         |
| Timestamp hoặc số rất lớn         | `i64` / `u64`   |
| Big Integer trong phạm vi 128 bit | `i128` / `u128` |

- types: bool | char | arr | tuple
- &value: Tham chiếu

`use std::io::{self, Write}; // io là module phục vụ in/out `

`use std::fs; // fs là module làm việc với file`

`unwrap(); // unwrap để lấy giá trị, lỗi sẽ trả called`
`Result::unwrap() on an Err value: Os {`
`code: 2,`
` kind: NotFound,`
`message: "No such file or directory"`
`}`

- expect: value || "msg" err // Khuyên sử dụng hơn unwrap()

- Ordering: enum biểu diễn phép so sánh

- let : - k thay đổi được
- let mut: - thay đổi được
- const : - Khai báo hằng số

- a[index] \* Index hợp lệ → lấy giá trị. \* Index không hợp lệ → panic.
- a.get(index) \* không panic, trả về Option, buộc bạn xử lý trường hợp không có phần tử

- Vec<T> → danh sách động (transactions, accounts, events...). -> always used
- Array [T; N] → dữ liệu cố định (ví dụ 32 bytes của public key, hash, signature). -> N: Numbers of elements , T: type
-
