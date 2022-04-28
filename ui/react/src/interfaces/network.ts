export type NetworkOptions = 'uni-2';

export interface Network {
  readonly chainId: string;
  readonly chainName: string;
  readonly addressPrefix: string;
  readonly rpcUrl: string;
  readonly httpUrl: string;
  readonly faucetUrl?: string;
  readonly faucetToken?: string;
  readonly feeToken: string;
  readonly stakingToken: string;
  readonly coinMap: { [key: string]: { denom: string; fractionalDigits: number } };
  readonly gasPrice: number;
  readonly codeId?: number;
}
