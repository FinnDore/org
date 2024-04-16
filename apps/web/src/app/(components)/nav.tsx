"use session";
import { Button } from "@/components/ui/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuShortcut,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Pfp } from "@/components/ui/pfp";
import { useDebounceValue } from "@/lib/utils";
import { ExitIcon } from "@radix-ui/react-icons";
import { signIn, signOut, useSession } from "next-auth/react";
import { useTheme } from "next-themes";
import Link from "next/link";
import { DynamicIsland } from "./dynamic-island";

export function Nav() {
    return (
        <nav className="grid grid-cols-3 justify-between p-4 px-8">
            <Link href="/" prefetch={false} className="my-auto">
                <h1 className="text-xl font-bold uppercase">Org</h1>
            </Link>
            <div className="flex justify-center">
                <DynamicIsland />
            </div>
            <User />
        </nav>
    );
}

function User() {
    const session = useSession();
    const loading = useDebounceValue(session.status === "loading");
    const theme = useTheme();
    const nextTheme =
        theme.theme === "light"
            ? "dark"
            : theme.theme === "dark"
              ? "system"
              : theme.theme === "system"
                ? "light"
                : "light";

    return (
        <div className="relative flex h-8 justify-end">
            {session.data && !loading && (
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <button>
                            <Pfp user={session.data.user} className="h-8" />
                        </button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                        <Link href="/me">
                            <DropdownMenuItem>profile</DropdownMenuItem>
                        </Link>
                        <DropdownMenuItem
                            onClick={() => theme.setTheme(nextTheme)}
                        >
                            theme: {theme.theme}
                        </DropdownMenuItem>

                        <DropdownMenuSeparator />
                        <DropdownMenuItem onClick={() => void signOut()}>
                            logout
                            <DropdownMenuShortcut>
                                <ExitIcon />
                            </DropdownMenuShortcut>
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            )}
            {session.status === "unauthenticated" && !loading && (
                <Button
                    variant="link"
                    className="text-lg"
                    onClick={() => void signIn("discord")}
                >
                    login
                </Button>
            )}
        </div>
    );
}

{
}
