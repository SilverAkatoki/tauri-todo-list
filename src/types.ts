export interface TaskType {
    description: string;
    isCompleted: boolean;
};

export type Option<T> = T | null;

export interface ClipboardType {
    title: string;
    tasks: TaskType[];
};
