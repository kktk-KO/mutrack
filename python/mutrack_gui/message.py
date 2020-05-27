from struct import Struct
from struct import calcsize

class Message(object):
    struct = Struct('>QQQQII')

    def __init__(self, start_time, finish_time, tid, message_type):
        self.start_time = start_time
        self.finish_time = finish_time
        self.tid = tid
        self.message_type = message_type

    def __str__(self):
        return '<MessagType message_type = {}, tid = {}, start_time = {}, finish_time = {}>'.format(
            self.message_type,
            self.tid,
            self.start_time,
            self.finish_time,
        )

    def __repr__(self):
        return self.__str__()

    @classmethod
    def deserialize(cls, f):
        t0a, t0b, t1a, t1b, tid, message_type = cls.struct.unpack_from(f.read(cls.struct.size))
        return Message(t0a << 64 | t0b, t1a << 64 | t1b, tid, message_type)

    @classmethod
    def deserialize_all(cls, f):
        messages = []
        while True:
            try:
                messages.append(cls.deserialize(f))
            except:
                break
        return messages
