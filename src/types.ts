interface TaskType {
    description: string;
    isCompleted: boolean;
};

export interface ClipboardType {
    title: string;
    tasks: TaskType[];
};
