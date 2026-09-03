<script lang="ts">
    import * as Select from "$lib/components/ui/select/index.js";

    let { approvalStatus = $bindable(), allowReject = false } = $props<{
        approvalStatus: string;
        allowReject?: boolean;
    }>();

    const statuses = $derived([
        { value: "0", label: "Pending" },
        { value: "1", label: "Approved" },
        ...(allowReject ? [{ value: "2", label: "Rejected" }] : []),
    ]);

    const triggerContent = $derived(
        statuses.find((f) => f.value === approvalStatus)?.label ??
            statuses[0].label,
    );
</script>

<Select.Root type="single" name="isApproved" bind:value={approvalStatus}>
    <Select.Trigger class="w-45">
        {triggerContent}
    </Select.Trigger>
    <Select.Content>
        {#each statuses as status}
            <Select.Item value={status.value}>{status.label}</Select.Item>
        {/each}
    </Select.Content>
</Select.Root>
