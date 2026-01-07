// Copyright (c) 2017-2026 Cloudflare, Inc.
// Licensed under the Apache 2.0 license found in the LICENSE file or at:
//     https://opensource.org/licenses/Apache-2.0
import { createRequire } from 'node:module';
import { strictEqual, ok, doesNotThrow } from 'node:assert';

const require = createRequire('/');

export const testTimersPromisesMutable = {
  test() {
    const timersPromises = require('node:timers/promises');
    const originalSetImmediate = timersPromises.setImmediate;
    ok(typeof originalSetImmediate === 'function');

    const patchedSetImmediate = async function patchedSetImmediate() {
      return 'patched';
    };

    doesNotThrow(() => {
      timersPromises.setImmediate = patchedSetImmediate;
    });

    strictEqual(timersPromises.setImmediate, patchedSetImmediate);
    timersPromises.setImmediate = originalSetImmediate;
    strictEqual(timersPromises.setImmediate, originalSetImmediate);
  },
};

export const testTimersMutable = {
  test() {
    const timers = require('node:timers');
    const originalSetTimeout = timers.setTimeout;
    ok(typeof originalSetTimeout === 'function');

    const patchedSetTimeout = function patchedSetTimeout() {
      return 'patched';
    };

    doesNotThrow(() => {
      timers.setTimeout = patchedSetTimeout;
    });

    strictEqual(timers.setTimeout, patchedSetTimeout);
    timers.setTimeout = originalSetTimeout;
  },
};

export const testBufferMutable = {
  test() {
    const buffer = require('node:buffer');
    const originalBuffer = buffer.Buffer;
    ok(typeof originalBuffer === 'function');

    const patchedBuffer = function PatchedBuffer() {
      return 'patched';
    };

    doesNotThrow(() => {
      buffer.Buffer = patchedBuffer;
    });

    strictEqual(buffer.Buffer, patchedBuffer);
    buffer.Buffer = originalBuffer;
  },
};

export const testUtilMutable = {
  test() {
    const util = require('node:util');
    const originalPromisify = util.promisify;
    ok(typeof originalPromisify === 'function');

    const patchedPromisify = function patchedPromisify() {
      return 'patched';
    };

    doesNotThrow(() => {
      util.promisify = patchedPromisify;
    });

    strictEqual(util.promisify, patchedPromisify);
    util.promisify = originalPromisify;
  },
};

export const testRequireCachesMutableObject = {
  test() {
    const timersPromises1 = require('node:timers/promises');
    const timersPromises2 = require('node:timers/promises');

    strictEqual(timersPromises1, timersPromises2);

    const patchedSetImmediate = async function patched() {
      return 'patched';
    };
    const original = timersPromises1.setImmediate;

    timersPromises1.setImmediate = patchedSetImmediate;
    strictEqual(timersPromises2.setImmediate, patchedSetImmediate);
    timersPromises1.setImmediate = original;
  },
};
