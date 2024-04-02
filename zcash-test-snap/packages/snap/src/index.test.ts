import { expect } from '@jest/globals';
import { installSnap } from '@metamask/snaps-jest';
import { panel, text } from '@metamask/snaps-sdk';

describe('onRpcRequest', () => {
  jest.setTimeout(600000);
  // describe('hello', () => {
  //   it('shows a confirmation dialog', async () => {
  //     const { request } = await installSnap();

  //     const origin = 'Jest';
  //     const response = request({
  //       method: 'hello',
  //       origin,
  //     });

  //     const ui = await response.getInterface();
  //     expect(ui.type).toBe('confirmation');
  //     expect(ui).toRender(
  //       panel([
  //         text(`Hello, **${origin}**!`),
  //         text('This custom confirmation is just for display purposes.'),
  //         text(
  //           'But you can edit the snap source code to make it do something, if you want to!',
  //         ),
  //       ]),
  //     );

  //     await ui.ok();

  //     expect(await response).toRespondWith(true);
  //   });
  // });

  describe('wasm', () => {
    it('runs proof', async () => {
      const { request } = await installSnap();

      const origin = 'Jest';
      const response = await request({
        method: 'proof',
        origin,
      });

      expect(response).toRespondWith(3);
    });

  it('runs trial-decrypt', async () => {
    const { request } = await installSnap();

    const origin = 'Jest';
    const response = await request({
      method: 'trial-decrypt',
      origin,
    });

    expect(response).toRespondWith(11273);
  });
});

  it('throws an error if the requested method does not exist', async () => {
    const { request, close } = await installSnap();

    const response = await request({
      method: 'foo',
    });

    expect(response).toRespondWithError({
      code: -32603,
      message: 'Method not found.',
      stack: expect.any(String),
    });

    await close();
  });
});
