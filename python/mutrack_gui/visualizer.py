import matplotlib.pyplot as plt
import matplotlib.patches as patches

class Visualizer(object):

    def show(self, messages):
        data = {}
        fig = plt.figure()
        ax = plt.axes()
        for message in messages:
            if message.tid not in data:
                data[message.tid] = []
            data[message.tid].append(message)
        for messages in data.values():
            messages.sort(key=lambda x: x.start_time)
        max_x = 0

        keys = sorted(data.keys())

        for index, key in enumerate(keys):
            messages = data[key]
            state = { 'lock': None, 'locked': False }
            X = []
            for message in messages:
                print(message)
                if message.message_type == 0:
                    assert(not state['locked'])
                    state = { 'lock': message, 'locked': True }
                elif message.message_type == 1:
                    assert(state['locked'])
                    x0 = 1.0 * state['lock'].start_time / 1e6
                    y0 = index + 0.25
                    x1 = 1.0 * state['lock'].finish_time / 1e6
                    y1 = index + 0.75
                    x2 = 1.0 * message.start_time / 1e6
                    y2 = index + 0.75

                    max_x = max(max_x, x1)
                    ax.add_patch(patches.Rectangle(
                        xy=(x0, y0),
                        width=x1-x0,
                        height=y1-y0,
                        fill=True,
                        facecolor='red',
                        edgecolor='red',
                        zorder=1.0,
                    ))
                    ax.add_patch(patches.Rectangle(
                        xy=(x1, y0),
                        width=x2-x1,
                        height=y2-y0,
                        fill=True,
                        facecolor='blue',
                        edgecolor='blue',
                        zorder=0.0,
                    ))
                    state = { 'lock': None, 'locked': False }
        ax.set_xlim(0, max_x)
        ax.set_ylim(0, index + 1)
        plt.show()
