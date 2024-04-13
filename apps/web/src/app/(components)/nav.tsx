"use session";
import { useSession } from "next-auth/react";
import { useEffect, useRef, useState } from "react";

export function Nav() {
    return (
        <nav className="flex justify-between p-4 px-8">
            <h1 className="text-xl font-bold uppercase">Org</h1>
            <div className="ms-auto flex gap-4">
                <User />
            </div>
        </nav>
    );
}

function User() {
    const session = useSession();

    const loading = useDebounceValue(session.status === "loading", {
        defaultValue: false,
    });
    return (
        <div className="h-10">
            {session.data && !loading && (
                <img
                    src={session.data.user.image!}
                    alt="user"
                    className="h-full rounded-full border border-black/40 dark:border-white/40"
                />
            )}
            {loading && <button>login</button>}
        </div>
    );
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

{
    /* <ThemeToggle />; */
}
