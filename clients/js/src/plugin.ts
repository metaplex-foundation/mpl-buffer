import { UmiPlugin } from '@metaplex-foundation/umi';
import { createMplBufferProgram } from './generated';

export const mplBuffer = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createMplBufferProgram(), false);
  },
});
