export interface Todo {
  id: number;
  description: string;
  status: TodoStatus;
}

export enum TodoStatus {
  CLOSED = 'CLOSED',
  OPEN = 'OPEN',
  COMPLETED = 'COMPLETED'
}
