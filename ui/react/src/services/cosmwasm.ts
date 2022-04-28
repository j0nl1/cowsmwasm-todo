import { ExecuteResult, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { OfflineSigner } from '@cosmjs/launchpad';
import { GasPrice } from '@cosmjs/stargate';
import { Todo, TodoStatus } from '../interfaces/todo';
import networks from '../configs/networks';

const network = networks['uni-2'];

const CONTRACT_ADDR = 'juno1p3ge8ekrm7g3v9gz5p4wqjjuac6svj852zum8pxga6j453vkxhps36m7xe';

export interface TodoClient {
  account: string;
  getTodo: (id: number, addr: string) => Promise<Todo>;
  getTodoList: (addr: string) => Promise<Todo[]>;
  addTodo: (description: string) => Promise<ExecuteResult>;
  changeTodoStatus: (id: number, status: TodoStatus) => Promise<ExecuteResult>;
  deleteTodo: (id: number) => Promise<ExecuteResult>;
}

export const createSignClient = async (signer: OfflineSigner): Promise<TodoClient> => {
  const [firstAccount] = await signer.getAccounts();
  const signingClient = await SigningCosmWasmClient.connectWithSigner(network.rpcUrl, signer, {
    prefix: network.addressPrefix,
    gasPrice: GasPrice.fromString(`${network.gasPrice}${network.feeToken}`)
  });

  const query = async (query: Record<string, unknown>) => signingClient.queryContractSmart(CONTRACT_ADDR, query);
  const execute = async (execution: Record<string, unknown>) => signingClient.execute(firstAccount.address, CONTRACT_ADDR, execution, 'auto');

  const getTodo = async (id: number, addr: string) => query({ get_todo: { id, addr } });
  const getTodoList = async (addr: string) => {
    const { todos } = await query({ get_list: { addr } });
    return todos?.map(([id, todo]: [number, {}]) => Object.assign(todo, { id }));
  };

  const addTodo = async (description: string) => execute({ add_todo: { description } });
  const changeTodoStatus = async (id: number, status: TodoStatus) => execute({ change_status: { id, status: 2 } });
  const deleteTodo = async (id: number) => execute({ delete: { id } });

  return { getTodo, getTodoList, addTodo, changeTodoStatus, deleteTodo, account: firstAccount.address };
};
