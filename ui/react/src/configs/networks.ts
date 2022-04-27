import { Network } from '../interfaces/network';

const uninet: Network = {
  chainId: 'uni-2',
  chainName: 'Juno (uni-2)',
  addressPrefix: 'juno',
  rpcUrl: 'https://rpc.uni.junonetwork.io/',
  httpUrl: 'https://api.uni.junonetwork.io/',
  feeToken: 'ujunox',
  stakingToken: 'ujunox',
  coinMap: {
    ujunox: { denom: 'JUNOX', fractionalDigits: 6 }
  },
  gasPrice: 0.025
};

const networks = {
  'uni-2': uninet
};

export default networks;
