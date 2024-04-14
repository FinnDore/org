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
import { useRouter } from "next/navigation";

export function Nav() {
    return (
        <nav className="flex justify-between p-4 px-8">
            <Link href="/" prefetch={false}>
                <h1 className="text-xl font-bold uppercase">Org</h1>
            </Link>
            <User />
        </nav>
    );
}

function User() {
    const session = useSession();
    const router = useRouter();

    const loading = useDebounceValue(session.status === "loading", {
        defaultValue: false,
    });
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
        <div className="relative h-8">
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
            {!session.data && !loading && (
                <Button
                    variant="link"
                    className="text-lg"
                    onClick={() => void signIn("discord")}
                >
                    login
                </Button>
            )}
            {loading && <button>login</button>}
        </div>
    );
}

{
    /* <ThemeToggle />; */
}
