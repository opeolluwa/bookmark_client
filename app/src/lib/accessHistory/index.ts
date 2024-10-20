import { Device } from "../device";

export default class InformationAccessHistory {
  device: Device;
  date: string;

  constructor(device: Device, date: string) {
    this.device = device;
    this.date = date;
  }
}
