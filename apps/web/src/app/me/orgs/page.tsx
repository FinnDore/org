"use client";

import { Button } from "@/components/ui/button";
import { cn, useDebounceValue } from "@/lib/utils";
import { api } from "@/trpc/react";
import Link from "next/link";

export default function Home() {
    const orgsQuery = api.me.orgs.useQuery();
    const loading = orgsQuery.status === "pending";
    const debouncedLoading = useDebounceValue(loading);
    return (
        <>
            <div className="mb-5 flex justify-between">
                <h1 className="text-xl font-bold">My Orgs</h1>
                <Link href="/me/orgs/new">
                    <Button>Create Org</Button>
                </Link>
            </div>
            <div className="relative mx-auto flex h-max w-full flex-col place-items-center justify-center gap-3 pb-8">
                {debouncedLoading && (
                    <>
                        <div className="flex h-[58px] w-full animate-pulse justify-center rounded-md border border-black/25 bg-black/10 dark:border-white/25 dark:bg-white/10"></div>
                        <div className="flex h-[58px] w-full animate-pulse justify-center rounded-md border border-black/25 bg-black/10 dark:border-white/25 dark:bg-white/10"></div>
                        <div className="flex h-[58px] w-full animate-pulse justify-center rounded-md border border-black/25 bg-black/10 dark:border-white/25 dark:bg-white/10"></div>
                        <div className="flex h-[58px] w-full animate-pulse justify-center rounded-md border border-black/25 bg-black/10 dark:border-white/25 dark:bg-white/10"></div>
                        <div className="flex h-[58px] w-full animate-pulse justify-center rounded-md border border-black/25 bg-black/10 dark:border-white/25 dark:bg-white/10"></div>
                        <div className="flex h-[58px] w-full animate-pulse justify-center rounded-md border border-black/25 bg-black/10 dark:border-white/25 dark:bg-white/10"></div>
                        <div className="flex h-[58px] w-full animate-pulse justify-center rounded-md border border-black/25 bg-black/10 dark:border-white/25 dark:bg-white/10"></div>
                        <div className="flex h-[58px] w-full animate-pulse justify-center rounded-md border border-black/25 bg-black/10 dark:border-white/25 dark:bg-white/10"></div>
                        <div className="absolute top-0 h-full w-full bg-gradient-to-t from-white to-white/40 dark:from-black dark:to-black/40"></div>
                    </>
                )}
                {!loading && (orgsQuery.data?.length ?? 0) < 1 && (
                    <div>You have no orgs orgs</div>
                )}
                {!debouncedLoading &&
                    orgsQuery.data?.map((org) => (
                        <Link
                            key={org.id}
                            className={cn(
                                "relative flex w-full gap-3 rounded-md border border-black/50 bg-white px-6 py-4 transition-all hover:border-black dark:border-white/50 dark:bg-black dark:hover:border-white",
                            )}
                            href={`/org/${org.id}`}
                        >
                            <h2>{org.name}</h2>
                            <p>{org.description}</p>
                        </Link>
                    ))}
            </div>
        </>
    );
}
