export interface Day {
    name: string;
    enabled: boolean;
    date: Date;
    item: Item | null;
}

export interface Item {
    title: string;
    classes: string;
}