/**
 * @fileoverview gRPC-Web generated client stub for cash.z.wallet.sdk.rpc
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.5.0
// 	protoc              v4.25.3
// source: service.proto


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var compact_formats_pb = require('./compact_formats_pb.js')
const proto = {};
proto.cash = {};
proto.cash.z = {};
proto.cash.z.wallet = {};
proto.cash.z.wallet.sdk = {};
proto.cash.z.wallet.sdk.rpc = require('./service_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname.replace(/\/+$/, '');

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname.replace(/\/+$/, '');

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.ChainSpec,
 *   !proto.cash.z.wallet.sdk.rpc.BlockID>}
 */
const methodDescriptor_CompactTxStreamer_GetLatestBlock = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLatestBlock',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.ChainSpec,
  proto.cash.z.wallet.sdk.rpc.BlockID,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.ChainSpec} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.BlockID.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.ChainSpec} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.BlockID)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.BlockID>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getLatestBlock =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLatestBlock',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetLatestBlock,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.ChainSpec} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.BlockID>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getLatestBlock =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLatestBlock',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetLatestBlock);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.BlockID,
 *   !proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 */
const methodDescriptor_CompactTxStreamer_GetBlock = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlock',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.BlockID,
  compact_formats_pb.CompactBlock,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  compact_formats_pb.CompactBlock.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.CompactBlock)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.CompactBlock>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getBlock =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlock',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetBlock,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getBlock =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlock',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetBlock);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.BlockID,
 *   !proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 */
const methodDescriptor_CompactTxStreamer_GetBlockNullifiers = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockNullifiers',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.BlockID,
  compact_formats_pb.CompactBlock,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  compact_formats_pb.CompactBlock.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.CompactBlock)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.CompactBlock>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getBlockNullifiers =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockNullifiers',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetBlockNullifiers,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getBlockNullifiers =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockNullifiers',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetBlockNullifiers);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.BlockRange,
 *   !proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 */
const methodDescriptor_CompactTxStreamer_GetBlockRange = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockRange',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.cash.z.wallet.sdk.rpc.BlockRange,
  compact_formats_pb.CompactBlock,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.BlockRange} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  compact_formats_pb.CompactBlock.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockRange} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getBlockRange =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockRange',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetBlockRange);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockRange} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getBlockRange =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockRange',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetBlockRange);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.BlockRange,
 *   !proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 */
const methodDescriptor_CompactTxStreamer_GetBlockRangeNullifiers = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockRangeNullifiers',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.cash.z.wallet.sdk.rpc.BlockRange,
  compact_formats_pb.CompactBlock,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.BlockRange} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  compact_formats_pb.CompactBlock.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockRange} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getBlockRangeNullifiers =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockRangeNullifiers',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetBlockRangeNullifiers);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockRange} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.CompactBlock>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getBlockRangeNullifiers =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetBlockRangeNullifiers',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetBlockRangeNullifiers);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.TxFilter,
 *   !proto.cash.z.wallet.sdk.rpc.RawTransaction>}
 */
const methodDescriptor_CompactTxStreamer_GetTransaction = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTransaction',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.TxFilter,
  proto.cash.z.wallet.sdk.rpc.RawTransaction,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.TxFilter} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.RawTransaction.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.TxFilter} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.RawTransaction)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.RawTransaction>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getTransaction =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTransaction',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetTransaction,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.TxFilter} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.RawTransaction>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getTransaction =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTransaction',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetTransaction);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.RawTransaction,
 *   !proto.cash.z.wallet.sdk.rpc.SendResponse>}
 */
const methodDescriptor_CompactTxStreamer_SendTransaction = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/SendTransaction',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.RawTransaction,
  proto.cash.z.wallet.sdk.rpc.SendResponse,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.RawTransaction} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.SendResponse.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.RawTransaction} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.SendResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.SendResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.sendTransaction =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/SendTransaction',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_SendTransaction,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.RawTransaction} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.SendResponse>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.sendTransaction =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/SendTransaction',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_SendTransaction);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.TransparentAddressBlockFilter,
 *   !proto.cash.z.wallet.sdk.rpc.RawTransaction>}
 */
const methodDescriptor_CompactTxStreamer_GetTaddressTxids = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTaddressTxids',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.cash.z.wallet.sdk.rpc.TransparentAddressBlockFilter,
  proto.cash.z.wallet.sdk.rpc.RawTransaction,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.TransparentAddressBlockFilter} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.RawTransaction.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.TransparentAddressBlockFilter} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.RawTransaction>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getTaddressTxids =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTaddressTxids',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetTaddressTxids);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.TransparentAddressBlockFilter} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.RawTransaction>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getTaddressTxids =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTaddressTxids',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetTaddressTxids);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.AddressList,
 *   !proto.cash.z.wallet.sdk.rpc.Balance>}
 */
const methodDescriptor_CompactTxStreamer_GetTaddressBalance = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTaddressBalance',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.AddressList,
  proto.cash.z.wallet.sdk.rpc.Balance,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.AddressList} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.Balance.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.AddressList} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.Balance)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.Balance>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getTaddressBalance =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTaddressBalance',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetTaddressBalance,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.AddressList} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.Balance>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getTaddressBalance =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTaddressBalance',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetTaddressBalance);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.Exclude,
 *   !proto.cash.z.wallet.sdk.rpc.CompactTx>}
 */
const methodDescriptor_CompactTxStreamer_GetMempoolTx = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetMempoolTx',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.cash.z.wallet.sdk.rpc.Exclude,
  compact_formats_pb.CompactTx,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.Exclude} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  compact_formats_pb.CompactTx.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Exclude} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.CompactTx>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getMempoolTx =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetMempoolTx',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetMempoolTx);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Exclude} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.CompactTx>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getMempoolTx =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetMempoolTx',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetMempoolTx);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.Empty,
 *   !proto.cash.z.wallet.sdk.rpc.RawTransaction>}
 */
const methodDescriptor_CompactTxStreamer_GetMempoolStream = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetMempoolStream',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.cash.z.wallet.sdk.rpc.Empty,
  proto.cash.z.wallet.sdk.rpc.RawTransaction,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.RawTransaction.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.RawTransaction>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getMempoolStream =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetMempoolStream',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetMempoolStream);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.RawTransaction>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getMempoolStream =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetMempoolStream',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetMempoolStream);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.BlockID,
 *   !proto.cash.z.wallet.sdk.rpc.TreeState>}
 */
const methodDescriptor_CompactTxStreamer_GetTreeState = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTreeState',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.BlockID,
  proto.cash.z.wallet.sdk.rpc.TreeState,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.TreeState.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.TreeState)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.TreeState>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getTreeState =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTreeState',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetTreeState,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.BlockID} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.TreeState>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getTreeState =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetTreeState',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetTreeState);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.Empty,
 *   !proto.cash.z.wallet.sdk.rpc.TreeState>}
 */
const methodDescriptor_CompactTxStreamer_GetLatestTreeState = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLatestTreeState',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.Empty,
  proto.cash.z.wallet.sdk.rpc.TreeState,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.TreeState.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.TreeState)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.TreeState>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getLatestTreeState =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLatestTreeState',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetLatestTreeState,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.TreeState>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getLatestTreeState =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLatestTreeState',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetLatestTreeState);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.GetSubtreeRootsArg,
 *   !proto.cash.z.wallet.sdk.rpc.SubtreeRoot>}
 */
const methodDescriptor_CompactTxStreamer_GetSubtreeRoots = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetSubtreeRoots',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.cash.z.wallet.sdk.rpc.GetSubtreeRootsArg,
  proto.cash.z.wallet.sdk.rpc.SubtreeRoot,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.GetSubtreeRootsArg} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.SubtreeRoot.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.GetSubtreeRootsArg} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.SubtreeRoot>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getSubtreeRoots =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetSubtreeRoots',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetSubtreeRoots);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.GetSubtreeRootsArg} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.SubtreeRoot>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getSubtreeRoots =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetSubtreeRoots',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetSubtreeRoots);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg,
 *   !proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReplyList>}
 */
const methodDescriptor_CompactTxStreamer_GetAddressUtxos = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxos',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg,
  proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReplyList,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReplyList.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReplyList)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReplyList>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getAddressUtxos =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxos',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetAddressUtxos,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReplyList>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getAddressUtxos =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxos',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetAddressUtxos);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg,
 *   !proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReply>}
 */
const methodDescriptor_CompactTxStreamer_GetAddressUtxosStream = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxosStream',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg,
  proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReply,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReply.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReply>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getAddressUtxosStream =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxosStream',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetAddressUtxosStream);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosArg} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.GetAddressUtxosReply>}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getAddressUtxosStream =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetAddressUtxosStream',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetAddressUtxosStream);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.Empty,
 *   !proto.cash.z.wallet.sdk.rpc.LightdInfo>}
 */
const methodDescriptor_CompactTxStreamer_GetLightdInfo = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLightdInfo',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.Empty,
  proto.cash.z.wallet.sdk.rpc.LightdInfo,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.LightdInfo.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.LightdInfo)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.LightdInfo>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.getLightdInfo =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLightdInfo',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetLightdInfo,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Empty} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.LightdInfo>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.getLightdInfo =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/GetLightdInfo',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_GetLightdInfo);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.cash.z.wallet.sdk.rpc.Duration,
 *   !proto.cash.z.wallet.sdk.rpc.PingResponse>}
 */
const methodDescriptor_CompactTxStreamer_Ping = new grpc.web.MethodDescriptor(
  '/cash.z.wallet.sdk.rpc.CompactTxStreamer/Ping',
  grpc.web.MethodType.UNARY,
  proto.cash.z.wallet.sdk.rpc.Duration,
  proto.cash.z.wallet.sdk.rpc.PingResponse,
  /**
   * @param {!proto.cash.z.wallet.sdk.rpc.Duration} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.cash.z.wallet.sdk.rpc.PingResponse.deserializeBinary
);


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Duration} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.cash.z.wallet.sdk.rpc.PingResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.cash.z.wallet.sdk.rpc.PingResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerClient.prototype.ping =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/Ping',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_Ping,
      callback);
};


/**
 * @param {!proto.cash.z.wallet.sdk.rpc.Duration} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.cash.z.wallet.sdk.rpc.PingResponse>}
 *     Promise that resolves to the response
 */
proto.cash.z.wallet.sdk.rpc.CompactTxStreamerPromiseClient.prototype.ping =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/cash.z.wallet.sdk.rpc.CompactTxStreamer/Ping',
      request,
      metadata || {},
      methodDescriptor_CompactTxStreamer_Ping);
};


module.exports = proto.cash.z.wallet.sdk.rpc;

