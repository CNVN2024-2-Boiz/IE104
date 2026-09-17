# Cấu trúc chính trong dự án:

![](assets/architecture.drawio.png)

# Nghiệp vụ

## Thực thể

- User
- Post
- Comment
- Report

## Tính năng

### FE

- Tự động dịch bình luận, bài viết
- Thêm live view của google map
- Tự động lấy role / gán role
- Báo cáo bình luận

### BE

- Lấy thông tin người dùng
- Lấy thông tin bài viết
- Lấy thông tin bình luận

### INFRA

- Tự động xếp user vào nhóm 'mute' khi chưa hết timeout
- Áp dụng xóa mềm, sau một khoảng thời gian thì sẽ có cơ chế dọn rác

## Nghiệp vụ

- Admin ủy quyền moderator
- Ban / Mute
