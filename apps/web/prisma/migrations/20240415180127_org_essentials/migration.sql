CREATE TABLE `ORG_Org` (
    `id` BIGINT(20) NOT NULL,
    `ownerId` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,
    `description` VARCHAR(191) NOT NULL,

    UNIQUE KEY `ORG_Org_id_key`(`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;


CREATE TABLE `ORG_OrgMember` (
    `ownerId` VARCHAR(191) NOT NULL,
    `orgId` BIGINT NOT NULL,

    UNIQUE INDEX `ORG_OrgMember_ownerId_orgId_key`(`ownerId`, `orgId`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;