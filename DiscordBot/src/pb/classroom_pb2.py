# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: classroom.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0f\x63lassroom.proto\x12\rdry.classroom\"\x16\n\x07\x41uthUrl\x12\x0b\n\x03url\x18\x01 \x01(\t\"\x1d\n\x0eNewAuthRequest\x12\x0b\n\x03who\x18\x01 \x01(\t2Q\n\x0b\x41uthService\x12\x42\n\x07NewAuth\x12\x1d.dry.classroom.NewAuthRequest\x1a\x16.dry.classroom.AuthUrl\"\x00\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'classroom_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
  DESCRIPTOR._options = None
  _globals['_AUTHURL']._serialized_start=34
  _globals['_AUTHURL']._serialized_end=56
  _globals['_NEWAUTHREQUEST']._serialized_start=58
  _globals['_NEWAUTHREQUEST']._serialized_end=87
  _globals['_AUTHSERVICE']._serialized_start=89
  _globals['_AUTHSERVICE']._serialized_end=170
# @@protoc_insertion_point(module_scope)