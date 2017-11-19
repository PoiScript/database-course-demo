export class Staff {
  id: Number;
  tele: Number;
  name: String;
  address: String;
  staff_department: String;
}

export class JoinedStaff {
  staff: Staff;
  receipt_total: Number;
}
