from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Task(_message.Message):
    __slots__ = ["id", "title", "remindAt", "who"]
    ID_FIELD_NUMBER: _ClassVar[int]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    REMINDAT_FIELD_NUMBER: _ClassVar[int]
    WHO_FIELD_NUMBER: _ClassVar[int]
    id: str
    title: str
    remindAt: _timestamp_pb2.Timestamp
    who: str
    def __init__(self, id: _Optional[str] = ..., title: _Optional[str] = ..., remindAt: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., who: _Optional[str] = ...) -> None: ...

class Tasks(_message.Message):
    __slots__ = ["task"]
    TASK_FIELD_NUMBER: _ClassVar[int]
    task: _containers.RepeatedCompositeFieldContainer[Task]
    def __init__(self, task: _Optional[_Iterable[_Union[Task, _Mapping]]] = ...) -> None: ...

class CreateTaskRequest(_message.Message):
    __slots__ = ["title", "remindAt", "who"]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    REMINDAT_FIELD_NUMBER: _ClassVar[int]
    WHO_FIELD_NUMBER: _ClassVar[int]
    title: str
    remindAt: _timestamp_pb2.Timestamp
    who: str
    def __init__(self, title: _Optional[str] = ..., remindAt: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., who: _Optional[str] = ...) -> None: ...

class ListTaskRequest(_message.Message):
    __slots__ = ["who"]
    WHO_FIELD_NUMBER: _ClassVar[int]
    who: str
    def __init__(self, who: _Optional[str] = ...) -> None: ...

class DeleteTaskRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...
