import { MessageType } from './enums';

type Message = {
  type: MessageType;
  data: any;
};

export { Message };
