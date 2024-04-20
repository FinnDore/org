"use client";
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
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
import { usePathname } from "next/navigation";
import { create } from "zustand";

type OrgState = {
    orgName: string;
    setOrgName: (orgName: string) => void;
};
const useStore = create<OrgState>((set) => ({
    orgName: "verigoers",
    setOrgName: (orgName: string) => set(() => ({ orgName })),
}));

export function Nav() {
    return (
        <nav className="flex  p-4 px-0">
            <Link href="/" prefetch={false} className="my-auto">
                <h1 className="text-xl font-bold uppercase">Org</h1>
            </Link>

            <div className="my-auto ms-6">
                <BreadCrumbs />
            </div>

            <div className="ms-auto">
                <User />
            </div>
        </nav>
    );
}

export function BreadCrumbs() {
    const store = useStore();
    const path = usePathname();

    const segments = path.split("/").filter(Boolean);

    if (segments.length < 2) return;

    return (
        <Breadcrumb>
            <BreadcrumbList>
                {segments.map((segment, i) => {
                    const href = `/${segments.slice(0, i + 1).join("/")}`;
                    return (
                        <BreadcrumbItem key={segment}>
                            <BreadcrumbLink href={href}>
                                {segment}
                            </BreadcrumbLink>
                            {i < segments.length - 1 && <BreadcrumbSeparator />}
                        </BreadcrumbItem>
                    );
                })}
            </BreadcrumbList>
        </Breadcrumb>
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
        <div className="relative flex h-8">
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
