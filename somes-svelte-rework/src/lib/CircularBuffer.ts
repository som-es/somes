export class CircularBuffer<K, T> {
    buffer: T[];
    keys: K[];
    pointer: number;
    bufferLength: number;
    constructor(bufferLength: number) {
        this.buffer = [];
        this.keys = []
        this.pointer = 0;
        this.bufferLength = bufferLength;
    }

    push(key: K, element: T) {
        if (this.buffer.length === this.bufferLength) {
            this.buffer[this.pointer] = element;
            this.keys[this.pointer] = key;
        } else {
            this.buffer.push(element);
            this.keys.push(key);
        }
        this.pointer = (this.pointer + 1) % this.bufferLength;
    }

    find(key: K): T {
        return this.buffer[this.keys.findIndex((e) => e === key)];
    }
    
    findBy(predicate: (key: K) => boolean): T | undefined {
        const index = this.keys.findIndex(predicate);
        return index !== -1 ? this.buffer[index] : undefined;
    }
}