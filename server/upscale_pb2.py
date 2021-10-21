# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: upscale.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='upscale.proto',
  package='upscale',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\rupscale.proto\x12\x07upscale\"+\n\nResolution\x12\r\n\x05width\x18\x01 \x01(\r\x12\x0e\n\x06height\x18\x02 \x01(\r\"X\n\x0bResolutions\x12#\n\x06target\x18\x01 \x01(\x0b\x32\x13.upscale.Resolution\x12$\n\x07minimum\x18\x02 \x01(\x0b\x32\x13.upscale.Resolution\"\x9b\x01\n\x0eUpscaleRequest\x12\x0f\n\x05scale\x18\x01 \x01(\rH\x00\x12+\n\x0bresolutions\x18\x02 \x01(\x0b\x32\x14.upscale.ResolutionsH\x00\x12\x0f\n\x07\x64\x65noise\x18\x03 \x01(\x08\x12\x15\n\roriginal_file\x18\x04 \x01(\x0c\x12\x14\n\x0coriginal_ext\x18\x05 \x01(\tB\r\n\x0btarget_size\"E\n\x0fUpscaleResponse\x12 \n\x03res\x18\x01 \x01(\x0b\x32\x13.upscale.Resolution\x12\x10\n\x08upscaled\x18\x02 \x01(\x0c\x32K\n\tAwUpscale\x12>\n\x07Upscale\x12\x17.upscale.UpscaleRequest\x1a\x18.upscale.UpscaleResponse\"\x00\x62\x06proto3'
)




_RESOLUTION = _descriptor.Descriptor(
  name='Resolution',
  full_name='upscale.Resolution',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='width', full_name='upscale.Resolution.width', index=0,
      number=1, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='height', full_name='upscale.Resolution.height', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=26,
  serialized_end=69,
)


_RESOLUTIONS = _descriptor.Descriptor(
  name='Resolutions',
  full_name='upscale.Resolutions',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='target', full_name='upscale.Resolutions.target', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='minimum', full_name='upscale.Resolutions.minimum', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=71,
  serialized_end=159,
)


_UPSCALEREQUEST = _descriptor.Descriptor(
  name='UpscaleRequest',
  full_name='upscale.UpscaleRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='scale', full_name='upscale.UpscaleRequest.scale', index=0,
      number=1, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='resolutions', full_name='upscale.UpscaleRequest.resolutions', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='denoise', full_name='upscale.UpscaleRequest.denoise', index=2,
      number=3, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='original_file', full_name='upscale.UpscaleRequest.original_file', index=3,
      number=4, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='original_ext', full_name='upscale.UpscaleRequest.original_ext', index=4,
      number=5, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
    _descriptor.OneofDescriptor(
      name='target_size', full_name='upscale.UpscaleRequest.target_size',
      index=0, containing_type=None,
      create_key=_descriptor._internal_create_key,
    fields=[]),
  ],
  serialized_start=162,
  serialized_end=317,
)


_UPSCALERESPONSE = _descriptor.Descriptor(
  name='UpscaleResponse',
  full_name='upscale.UpscaleResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='res', full_name='upscale.UpscaleResponse.res', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='upscaled', full_name='upscale.UpscaleResponse.upscaled', index=1,
      number=2, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=319,
  serialized_end=388,
)

_RESOLUTIONS.fields_by_name['target'].message_type = _RESOLUTION
_RESOLUTIONS.fields_by_name['minimum'].message_type = _RESOLUTION
_UPSCALEREQUEST.fields_by_name['resolutions'].message_type = _RESOLUTIONS
_UPSCALEREQUEST.oneofs_by_name['target_size'].fields.append(
  _UPSCALEREQUEST.fields_by_name['scale'])
_UPSCALEREQUEST.fields_by_name['scale'].containing_oneof = _UPSCALEREQUEST.oneofs_by_name['target_size']
_UPSCALEREQUEST.oneofs_by_name['target_size'].fields.append(
  _UPSCALEREQUEST.fields_by_name['resolutions'])
_UPSCALEREQUEST.fields_by_name['resolutions'].containing_oneof = _UPSCALEREQUEST.oneofs_by_name['target_size']
_UPSCALERESPONSE.fields_by_name['res'].message_type = _RESOLUTION
DESCRIPTOR.message_types_by_name['Resolution'] = _RESOLUTION
DESCRIPTOR.message_types_by_name['Resolutions'] = _RESOLUTIONS
DESCRIPTOR.message_types_by_name['UpscaleRequest'] = _UPSCALEREQUEST
DESCRIPTOR.message_types_by_name['UpscaleResponse'] = _UPSCALERESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Resolution = _reflection.GeneratedProtocolMessageType('Resolution', (_message.Message,), {
  'DESCRIPTOR' : _RESOLUTION,
  '__module__' : 'upscale_pb2'
  # @@protoc_insertion_point(class_scope:upscale.Resolution)
  })
_sym_db.RegisterMessage(Resolution)

Resolutions = _reflection.GeneratedProtocolMessageType('Resolutions', (_message.Message,), {
  'DESCRIPTOR' : _RESOLUTIONS,
  '__module__' : 'upscale_pb2'
  # @@protoc_insertion_point(class_scope:upscale.Resolutions)
  })
_sym_db.RegisterMessage(Resolutions)

UpscaleRequest = _reflection.GeneratedProtocolMessageType('UpscaleRequest', (_message.Message,), {
  'DESCRIPTOR' : _UPSCALEREQUEST,
  '__module__' : 'upscale_pb2'
  # @@protoc_insertion_point(class_scope:upscale.UpscaleRequest)
  })
_sym_db.RegisterMessage(UpscaleRequest)

UpscaleResponse = _reflection.GeneratedProtocolMessageType('UpscaleResponse', (_message.Message,), {
  'DESCRIPTOR' : _UPSCALERESPONSE,
  '__module__' : 'upscale_pb2'
  # @@protoc_insertion_point(class_scope:upscale.UpscaleResponse)
  })
_sym_db.RegisterMessage(UpscaleResponse)



_AWUPSCALE = _descriptor.ServiceDescriptor(
  name='AwUpscale',
  full_name='upscale.AwUpscale',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_start=390,
  serialized_end=465,
  methods=[
  _descriptor.MethodDescriptor(
    name='Upscale',
    full_name='upscale.AwUpscale.Upscale',
    index=0,
    containing_service=None,
    input_type=_UPSCALEREQUEST,
    output_type=_UPSCALERESPONSE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
])
_sym_db.RegisterServiceDescriptor(_AWUPSCALE)

DESCRIPTOR.services_by_name['AwUpscale'] = _AWUPSCALE

# @@protoc_insertion_point(module_scope)