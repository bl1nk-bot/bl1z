# Security Policy / นโยบายด้านความปลอดภัย

## Supported Versions / เวอร์ชันที่ได้รับการสนับสนุน

We take security seriously. This document outlines our security policy and how to report vulnerabilities.

เราให้ความสำคัญกับความปลอดภัยอย่างสูงสุด หัวข้อนี้สรุปนโยบายด้านความปลอดภัยและวิธีการรายงานช่องโหว่ด้านความปลอดภัย

### Version Support / การสนับสนุนเวอร์ชัน

| Version / เวอร์ชัน | Supported / สถานะการสนับสนุน |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Reporting a Vulnerability / การรายงานช่องโหว่

If you discover a security vulnerability in bl1z, please help us by reporting it responsibly.

หากคุณพบช่องโหว่ด้านความปลอดภัยใน bl1z โปรดช่วยเหลือเราด้วยการรายงานอย่างมีความรับผิดชอบ

### How to Report / วิธีการรายงาน

**DO NOT** create public GitHub issues for security vulnerabilities.

**ห้าม** สร้าง GitHub issues แบบสาธารณะสำหรับช่องโหว่ด้านความปลอดภัย

Please report security vulnerabilities via email:
กรุณารายงานช่องโหว่ด้านความปลอดภัยผ่านอีเมล:

- **Email / อีเมล**: security@bl1z.dev
- **Subject / หัวข้อ**: `[SECURITY] Vulnerability Report`

### What to Include / ข้อมูลที่ควรระบุ

Please include the following information in your report:
กรุณาระบุข้อมูลต่อไปนี้ในรายงานของคุณ:

- **Description / คำอธิบาย**: Clear description of the vulnerability / อธิบายลักษณะของช่องโหว่ให้ชัดเจน
- **Impact / ผลกระทบ**: Potential impact and severity / ผลกระทบที่อาจเกิดขึ้นและระดับความรุนแรง
- **Steps to Reproduce / ขั้นตอนการเกิดปัญหา**: Detailed reproduction steps / รายละเอียดขั้นตอนการทำให้เกิดปัญหา
- **Proof of Concept / การพิสูจน์แนวคิด**: Code or steps demonstrating the vulnerability / โค้ดหรือขั้นตอนที่แสดงให้เห็นถึงช่องโหว่
- **Environment / สภาพแวดล้อม**: Rust version, OS, dependencies / เวอร์ชัน Rust, ระบบปฏิบัติการ, dependencies ที่ใช้งาน
- **Suggested Fix / แนวทางแก้ไขที่เสนอ**: If you have suggestions for fixing the issue / หากคุณมีข้อเสนอแนะในการแก้ไขปัญหานี้

### Response Timeline / ระยะเวลาการตอบกลับ

We will acknowledge your report within **48 hours** and provide a more detailed response within **7 days** indicating our next steps.

เราจะตอบรับรายงานของคุณภายใน **48 ชั่วโมง** และจะให้การตอบกลับที่ละเอียดขึ้นภายใน **7 วัน** เพื่อระบุขั้นตอนถัดไปของเรา

We will keep you informed about the progress throughout the vulnerability remediation process.

เราจะแจ้งให้คุณทราบเกี่ยวกับความคืบหน้าตลอดกระบวนการแก้ไขช่องโหว่

## Security Considerations / ข้อควรพิจารณาด้านความปลอดภัย

### Memory Safety / ความปลอดภัยของหน่วยความจำ

bl1z is built with Rust and benefits from Rust's memory safety guarantees:

bl1z พัฒนาด้วยภาษา Rust และได้รับประโยชน์จากการรับประกันความปลอดภัยของหน่วยความจำของ Rust:

- No buffer overflows / ไม่มีปัญหา Buffer overflows
- No use-after-free / ไม่มีปัญหา Use-after-free
- No null pointer dereferences / ไม่มีปัญหา Null pointer dereferences
- Thread safety when used correctly / ความปลอดภัยในการทำงานแบบเธรดเมื่อใช้งานอย่างถูกต้อง

### Input Validation / การตรวจสอบข้อมูลนำเข้า

- All user input is validated / ข้อมูลนำเข้าจากผู้ใช้ทั้งหมดจะได้รับการตรวจสอบความถูกต้อง
- Expression parsing includes bounds checking / การแยกส่วนนิพจน์รวมถึงการตรวจสอบขอบเขต
- Type system validation prevents invalid operations / การตรวจสอบระบบชนิดข้อมูลเพื่อป้องกันการดำเนินการที่ไม่ถูกต้อง
- Error messages avoid exposing sensitive information / ข้อความแสดงข้อผิดพลาดหลีกเลี่ยงการเปิดเผยข้อมูลที่ละเอียดอ่อน

### Dependencies / การพึ่งพาภายนอก

- We regularly audit dependencies for known vulnerabilities / เราทำการตรวจสอบ dependencies อย่างสม่ำเสมอเพื่อหาช่องโหว่ที่ทราบ
- Dependencies are kept up to date / Dependencies จะถูกปรับปรุงให้ทันสมัยอยู่เสมอ
- Only necessary and well-vetted dependencies are included / เลือกใช้งานเฉพาะ dependencies ที่มีความจำเป็นและผ่านการตรวจสอบมาอย่างดี

### Best Practices / แนวทางปฏิบัติที่ดีที่สุด

When using bl1z in production:

เมื่อใช้งาน bl1z ในสภาพแวดล้อมจริง:

1. **Validate Input / ตรวจสอบข้อมูลนำเข้า**: Always validate formulas before evaluation / ตรวจสอบความถูกต้องของสูตรนำเข้าก่อนการประเมินค่าเสมอ
2. **Limit Complexity / จำกัดความซับซ้อน**: Set appropriate limits on expression complexity / ตั้งค่าขีดจำกัดที่เหมาะสมสำหรับความซับซ้อนของนิพจน์
3. **Sandbox Execution / การรันในแซนด์บ็อกซ์**: Run in isolated environments if processing untrusted formulas / รันในสภาพแวดล้อมที่แยกต่างหากหากประมวลผลสูตรที่ไม่น่าเชื่อถือ
4. **Monitor Performance / เฝ้าดูประสิทธิภาพ**: Watch for unusual performance patterns / ตรวจสอบรูปแบบประสิทธิภาพที่ผิดปกติ
5. **Stay Updated / ปรับปรุงให้ทันสมัย**: Apply security updates promptly / ใช้การอัปเดตด้านความปลอดภัยอย่างทันท่วงที

## Security Features / คุณสมบัติด้านความปลอดภัย

### Safe Evaluation / การประเมินค่าอย่างปลอดภัย

- **Type Safety / ความปลอดภัยของชนิดข้อมูล**: Runtime type checking prevents invalid operations / การตรวจสอบชนิดข้อมูลในขณะรันไทม์ช่วยป้องกันการดำเนินการที่ไม่ถูกต้อง
- **Bounds Checking / การตรวจสอบขอบเขต**: Array and string operations always check bounds / การดำเนินการกับอาร์เรย์และข้อความจะมีการตรวจสอบขอบเขตเสมอ
- **Recursion Limits / ขีดจำกัดการเรียกซ้ำ**: Prevents infinite recursion attacks / ป้องกันการโจมตีแบบเรียกซ้ำไม่สิ้นสุด
- **Resource Limits / ขีดจำกัดทรัพยากร**: Configurable limits on evaluation complexity / ตั้งค่าขีดจำกัดความซับซ้อนของการประเมินค่าได้

### Error Handling / การจัดการข้อผิดพลาด

- **No Information Leakage / ไม่มีการรั่วไหลของข้อมูล**: Error messages don't expose internal state / ข้อความแสดงข้อผิดพลาดจะไม่เปิดเผยสถานะภายในของระบบ
- **Result Validation / การควบคุมผลลัพธ์**: Evaluation results are validated before return / ผลลัพธ์จากการประเมินค่าจะได้รับการตรวจสอบก่อนส่งกลับ
- **Logging / การบันทึกเหตุการณ์**: Security-relevant events can be logged / เหตุการณ์ที่เกี่ยวข้องกับความปลอดภัยสามารถบันทึกเก็บไว้ได้

### Code Quality / คุณภาพของโค้ด

- **No Unsafe Code / ไม่มีโค้ดที่ไม่ปลอดภัย**: No unsafe blocks in source code / ไม่มีการใช้งานบล็อก unsafe ในซอร์สโค้ด
- **Comprehensive Testing / การทดสอบที่ครอบคลุม**: High test coverage including edge cases / ความครอบคลุมของการทดสอบสูง รวมถึงกรณีขอบเขตต่างๆ
- **Static Analysis / การวิเคราะห์แบบสถิต**: Clippy and other tools catch potential issues / ใช้งาน Clippy และเครื่องมืออื่นเพื่อตรวจจับปัญหาที่อาจเกิดขึ้น
- **Code Review / การรีวิวโค้ด**: All changes undergo security review / การเปลี่ยนแปลงทั้งหมดต้องผ่านการรีวิวในแง่ความปลอดภัย

## Third-Party Security / ความปลอดภัยจากบุคคลที่สาม

### Dependency Reporting / การรายงาน Dependencies

We monitor and address security issues in our dependencies:

เราเฝ้าติดตามและแก้ไขปัญหาด้านความปลอดภัยใน dependencies ของเรา:

- **jiff**: Date and time library - actively maintained with good security track record / ไลบรารีจัดการวันที่และเวลา - ได้รับการดูแลอย่างต่อเนื่องพร้อมประวัติความปลอดภัยที่ดี
- **Rust Ecosystem / ระบบนิเวศของ Rust**: Benefits from Rust's security-focused design / ได้รับประโยชน์จากการออกแบบที่เน้นความปลอดภัยของ Rust

### Supply Chain Security / ความปลอดภัยของห่วงโซ่อุปทาน

- **Minimal Dependencies / Dependencies ขั้นต่ำ**: Only include necessary dependencies / รวมเฉพาะ dependencies ที่จำเป็นเท่านั้น
- **Pinned Versions / การล็อกเวอร์ชัน**: Dependencies locked to specific versions / Dependencies ถูกล็อกไว้ที่เวอร์ชันเฉพาะ
- **Regular Updates / การอัปเดตสม่ำเสมอ**: Dependencies regularly updated with security patches / Dependencies จะได้รับการอัปเดตพร้อมแพตช์ความปลอดภัยอย่างสม่ำเสมอ

## Incident Response / การตอบสนองต่อเหตุการณ์

In the event of a confirmed security vulnerability:

ในกรณีที่ยืนยันพบช่องโหว่ด้านความปลอดภัย:

1. **Immediate Assessment / การประเมินทันที**: Assess impact and severity / ประเมินผลกระทบและระดับความรุนแรง
2. **Develop Fix / การพัฒนาการแก้ไข**: Develop and test security fix / พัฒนาและทดสอบการแก้ไขด้านความปลอดภัย
3. **Coordinated Release / การเปิดตัวที่มีการประสานงาน**: Release fix with vulnerability details / เปิดตัวการแก้ไขพร้อมรายละเอียดของช่องโหว่
4. **Communication / การสื่อสาร**: Notify users through appropriate channels / แจ้งผู้ใช้ผ่านช่องทางที่เหมาะสม

## Contact Information / ข้อมูลการติดต่อ

For security-related questions or concerns:

สำหรับคำถามหรือข้อกังวลที่เกี่ยวข้องกับความปลอดภัย:

- **Security Issues / ประเด็นความปลอดภัย**: security@bl1z.dev
- **General Support / การสนับสนุนทั่วไป**: support@bl1z.dev
- **GitHub Issues**: For non-security issues only / สำหรับประเด็นที่ไม่เกี่ยวกับความปลอดภัยเท่านั้น

## Acknowledgments / กิตติกรรมประกาศ

We appreciate the security research community's help in keeping open source software secure. Responsible disclosure is valued and recognized by us.

เราขอขอบคุณชุมชนการวิจัยด้านความปลอดภัยที่ช่วยรักษาความปลอดภัยให้กับซอฟต์แวร์โอเพนซอร์ส การเปิดเผยข้อมูลอย่างมีความรับผิดชอบนั้นมีค่าและได้รับการยอมรับจากเรา
