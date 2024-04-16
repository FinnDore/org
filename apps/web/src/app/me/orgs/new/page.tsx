"use client";

import { Button } from "@/components/ui/button";
import { api } from "@/trpc/react";
import { useRouter } from "next/navigation";

export default function Home() {
    const orgsQuery = api.me.createOrg.useMutation();
    const router = useRouter();
    return (
        <>
            <Button
                onClick={async () => {
                    await orgsQuery.mutateAsync({
                        name: "My org",
                        description: "My org description",
                    });

                    router.push("/me/orgs");
                }}
            >
                Create
            </Button>
        </>
    );
}
