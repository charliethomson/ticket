interface User {
  name: string;
  id: number;
  status: string;
  ban: boolean;
  queue: number[]; // IDs of active workorders assigned to this user
}

interface Workorder {
  customer: Customer;
  device: Device;
  status: number; // See STATUS
  notes: Note[];
  sale_items: Item[]; // TODO: Web assembly u64-ing
  location: string;
  id: number;
  assigned_user: number; // ID of the user assigned to the workorder

  is_active: boolean;
  is_warranty: boolean;

  next_update: Date;
  last_update: Date;
  created: Date;
}

type Item = number; // TODO

interface Note {
  author: User;
  created: Date;
  next_update: Date;
  status: number;
  // public: boolean, // TODO
}

interface Customer {}
interface Device {}

const STATUS = [
  "Sale completed",
  "Device abandoned",
  "Repaired - RFP",
  "Unrepairable - RFP",
  "No issue - RFP",
  "Declined - RFP",
  "Awaiting Diagnostic",
  "Awaiting Repair",
  "Awaiting Device",
  "Awaiting Customer",
  "Awaiting Technician",
  "Awaiting Parts",
  "Need to order parts",
];
