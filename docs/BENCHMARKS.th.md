# รายงานการทดสอบประสิทธิภาพของ bl1z (Benchmark Report - 2026-06-17)

เกณฑ์มาตรฐานประสิทธิภาพ (Performance baseline) สำหรับ bl1z v0.2.12

## สรุปผล (Summary)

| รายการทดสอบ (Benchmark) | เวลาในการประมวลผล | หมายเหตุ (Notes) |
|-----------|----------------|-------|
| `basic_arithmetic` | 1.42 µs | Tokenize + Parse + Eval |
| `complex_expression` | 7.65 µs | `if`, `sum`, `upper` |
| `large_array_sum` | 64.19 µs | ขนาดอาร์เรย์: 100 |
| `nested_functions` | 5.41 µs | `upper(join([hello, lower(world)]))` |
| `date_operations` | 3.01 µs | `year(date_add(...))` |
| `map_operations` | 3.16 µs | ค่าคงที่ `{a: 1, b: 2, c: 3}` |
| `phase8_access_chaining` | 4.68 µs | `user.profile.score + [10,20][1]` |
| `phase9_map_filter` | 15.68 µs | ลอจิกฟังก์ชันระดับสูง |
| `udf_call` | 1.66 µs | การเรียกใช้ฟังก์ชันที่ผู้ใช้นิยามเอง |
| `lambda_call` | 0.83 µs | การเรียกใช้ lambda ที่ประเมินค่าไว้ล่วงหน้า |
| `without_cache` | 5.79 µs | การประมวลผลเต็มรูปแบบ |
| `with_cache_hit` | 0.65 µs | เฉพาะการประเมินค่า (เร็วขึ้น 9 เท่า) |
| `set_operations` | 10.30 µs | `set_intersection` |
| `range_to_array` | 5.23 µs | ช่วงข้อมูล (Range) 0..100 |

## ข้อมูลเชิงลึกด้านประสิทธิภาพ (Performance Insights)

1. **การทำแคชมีประสิทธิภาพสูงมาก (Caching is highly effective)**: การใช้งาน `FormulaCache` ช่วยให้ประมวลผลเร็วขึ้นประมาณ 9 เท่าสำหรับสูตรที่มีการใช้งานซ้ำ โดยการข้ามขั้นตอนการวิเคราะห์คำศัพท์ (lexing) และการวิเคราะห์ไวยากรณ์ (parsing)
2. **Lambda เทียบกับ UDF**: การเรียกใช้ Lambda ในปัจจุบันเร็วกว่าการเรียกใช้ UDF ประมาณ 2 เท่า ซึ่งบ่งชี้ถึงโอกาสในการปรับแต่งประสิทธิภาพของตรรกะการสลับบริบท (context switching) ใน UDF
3. **ภาระการประมวลผลที่เพิ่มขึ้น (Complexity Overhead)**: การรองรับชนิดข้อมูลขั้นสูงและการเข้าถึงข้อมูลแบบลำดับชั้นทำให้เกิดภาระการประมวลผลเพิ่มขึ้นเล็กน้อย (ประมาณ 8-15%) เมื่อเทียบกับฐานข้อมูล V1 ซึ่งถือว่าอยู่ในระดับที่ยอมรับได้เมื่อแลกกับความสามารถที่เพิ่มขึ้น

## รายงานที่สร้างขึ้น (Generated Reports)

รายงานรูปแบบ HTML ฉบับเต็มพร้อมกราฟแบบโต้ตอบสามารถดูได้ที่:
`target/criterion/report/index.html`
