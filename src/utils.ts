export const debounce = (fn: Function, delay: number) => {
    let timer: number | null = null;
    return (...args: any[]) => {
        if (timer) {
            clearTimeout(timer);
        }
        timer = setTimeout(() => {
            fn(...args);
            timer = null;
        }, delay);
    };
};