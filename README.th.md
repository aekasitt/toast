# โทสต์

![Toast banner](static/toast-banner.svg)

### ภาษาที่รองรับ

* [English 🇬🇧](./README.md)
* [Thai 🇹🇭](./README.th.md)

### ข้อกำหนดเบื้องต้น

- [cURL](https://curl.se) - เครื่องมือและไลบรารีบรรทัดคำสั่งสำหรับถ่ายโอนข้อมูลด้วย URL
    <details>
      <summary> ติดตั้งด้วย Homebrew (Darwin) </summary>
      
      brew install curl
    </details>
    <details>
      <summary> ติดตั้งด้วย Advanced Package Tool (Linux และ Windows Subsystem for Linux [WSL]) </summary>
    
      apt update && apt upgrade
      apt install curl
    </details>

- [Rust](https://rust-lang.org) - ภาษาที่มอบพลังให้ทุกคนสร้างซอฟต์แวร์ที่เชื่อถือได้และมีประสิทธิภาพ
    <details>
      <summary> ติดดั้งด้วยอินสตอลเลอร์แบบยืนเดี่ยว (Darwin, Linux, และ Windows Subsystem for Linux [WSL]) </summary>
      
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    </details>

- [Just](https://just.systems) - แค่คอมมานด์รันคำสั่งตัวหนึ่ง
    <details>
      <summary> ติดตั้งด้วยคอมมานด์ไลน์ cargo แนบมากับ rustup </summary>
      
      cargo install just
    </details>

- [Jq](https://jqlang.org) - คิดแยกแยะวิเคราะห์ JSON ด้วยคอมมานด์ไลน์ที่เบาและยืดหยุ่น
    <details>
      <summary> ติดตั้งด้วย Homebrew (Darwin) </summary>
      
      brew install jq
    </details>
    <details>
      <summary> ติดตั้งด้วย Advanced Package Tool (Linux และ Windows Subsystem for Linux [WSL]) </summary>
    
      apt update && apt upgrade
      apt install jq
    </details>

### เริ่มต้น

เรากำลังสร้างแอปพลิเคชัน CRUD เพื่อจัดการรายการ Todo คล้าย Post-It 
เพื่อให้แผน C-Create, R-Read, U-Update และ D-Delete ของแอปพลิเคชันนี้สมบูรณ์ 
ผู้ใช้ต้องสามารถ

- สร้าง Todo ใหม่ที่คงอยู่ในบันทึกจัดเก็บ
- อ่านรายการ Todo ที่คงอยู่
- อัปเดตสถานะเสร็จสิ้นของ Todo
- ลบ Todo ออกจากรายการ Todo ที่มีอยู่


## กิตติกรรมประกาศ

* [บทความ] [Build a Single-File Rust Web API with SQLite](https://hamy.xyz/blog/2026-03_rust-webapi-db)
  โดยคุณ [แฮมิลตัน กรีน](https://hamy.xyz/)
* [บทความ] [Toasty, an async ORM for Rust, is now on crates.io](https://tokio.rs/blog/2026-04-03-toasty-released)
  โดยทีม [โทคิโอะ](https://tokio.rs) ในวันที่ 3 เมษายนปีคศ. 2026
* ฟอนต์ [EDMuzashi](https://www.f0nt.com/release/edmuzazhi)
  โดยคุณ [อาทรเกติ์ แสงเพชร](https://www.facebook.com/ed.crub)
  ฉายา [ed_crub](https://www.f0nt.com/by/ed_crub)

## ใบอนุญาต

โครงการนี้ได้รับอนุญาตเผยแพร่ภายใต้ใบอนุญาต MIT
