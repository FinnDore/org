import { clsx, type ClassValue } from "clsx";
import { useEffect, useRef, useState } from "react";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

function debounceTimeLeft(debounceTime: number, latSetTime: Date) {
    return Math.abs(Date.now() - (latSetTime.getTime() + debounceTime));
}

export function useDebounceValue<T extends number | string | boolean>(
    value: T | null,
    {
        delay = 250,
        defaultValue = null,
    }: {
        delay?: number;
        defaultValue: T | null;
    } = { delay: 250, defaultValue: null },
) {
    const [debouncedValue, setVal] = useState<T | null>(defaultValue);
    const lastSet = useRef<Date>(new Date(new Date().getTime()));
    const timeout = useRef<NodeJS.Timeout>();

    useEffect(() => {
        clearTimeout(timeout.current);
        if (!debounceTimeLeft(delay, lastSet.current)) {
            setVal(value);
            lastSet.current = new Date();
        } else {
            timeout.current = setTimeout(
                () => {
                    lastSet.current = new Date();
                    setVal(value);
                },
                debounceTimeLeft(delay, lastSet.current),
            );
        }

        return () => {
            clearTimeout(timeout.current);
        };
    }, [delay, value]);

    return debouncedValue;
}
