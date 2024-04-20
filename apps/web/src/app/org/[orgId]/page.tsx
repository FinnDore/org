"use client";
import { api } from "@/trpc/react";

export default function Org(props: { params: { orgId: string } }) {
    const org = api.org.get.useQuery(
        {
            orgId: +props.params.orgId,
        },
        {
            enabled: !isNaN(+props.params.orgId),
        },
    );

    return (
        <div>
            {org.data?.name} :: {org.data?.description}
        </div>
    );
}
