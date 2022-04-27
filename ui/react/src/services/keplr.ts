import { OfflineSigner } from '@cosmjs/launchpad';
import { Network, NetworkOptions } from '../interfaces/network';
import networks from '../configs/networks';

export const loadKeplr = async (networkName: NetworkOptions) => {
  if (!window || !window.keplr) return;
  try {
    await window.keplr.enable(networkName);
  } catch (err) {
    await addNetwork(networkName);
    await window.keplr.enable(networkName);
  }
};

export const getSigner = () => window?.keplr?.getOfflineSignerAuto('uni-2') as Promise<OfflineSigner>;

const addNetwork = async (networkName: NetworkOptions) => {
  const network = networks[networkName];
  const config = configKeplr(network);
  await window?.keplr?.experimentalSuggestChain(config);
};

const configKeplr = (config: Network) => {
  return {
    chainId: config.chainId,
    chainName: config.chainName,
    rpc: config.rpcUrl,
    rest: config.httpUrl,
    bech32Config: {
      bech32PrefixAccAddr: `${config.addressPrefix}`,
      bech32PrefixAccPub: `${config.addressPrefix}pub`,
      bech32PrefixValAddr: `${config.addressPrefix}valoper`,
      bech32PrefixValPub: `${config.addressPrefix}valoperpub`,
      bech32PrefixConsAddr: `${config.addressPrefix}valcons`,
      bech32PrefixConsPub: `${config.addressPrefix}valconspub`
    },
    currencies: [
      {
        coinDenom: config.coinMap[config.feeToken].denom,
        coinMinimalDenom: config.feeToken,
        coinDecimals: config.coinMap[config.feeToken].fractionalDigits
      },
      {
        coinDenom: config.coinMap[config.stakingToken].denom,
        coinMinimalDenom: config.stakingToken,
        coinDecimals: config.coinMap[config.stakingToken].fractionalDigits
      }
    ],
    feeCurrencies: [
      {
        coinDenom: config.coinMap[config.feeToken].denom,
        coinMinimalDenom: config.feeToken,
        coinDecimals: config.coinMap[config.feeToken].fractionalDigits
      }
    ],
    stakeCurrency: {
      coinDenom: config.coinMap[config.stakingToken].denom,
      coinMinimalDenom: config.stakingToken,
      coinDecimals: config.coinMap[config.stakingToken].fractionalDigits
    },
    gasPriceStep: {
      low: config.gasPrice / 2,
      average: config.gasPrice,
      high: config.gasPrice * 2
    },
    bip44: { coinType: 118 },
    coinType: 118
  };
};
