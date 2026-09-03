<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Form from "$lib/components/ui/form/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { formSchema } from "./schema";
    import { defaults, setError, superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "svelte-sonner";
    import DatePicker from "./component/date-picker.svelte";
    import TimePicker from "./component/time-picker.svelte";
    import ApprovalSelect from "./component/approval-select.svelte";
    import ColorPicker from "./component/color-picker.svelte";

    let approvalStatus = $state("0");
    let color = $state("#3B82F6");
    let isOpen = $state(false);

    const { onSuccess } = $props<{ onSuccess: () => void }>();

    const form = superForm(defaults(zod4(formSchema)), {
        validators: zod4(formSchema),
        SPA: true,
        async onUpdate({ form: formResult }) {
            if (!formResult.valid) return;

            if (
                new Date(
                    `${formResult.data.startDate} ${formResult.data.startTime}`,
                ) >
                new Date(
                    `${formResult.data.endDate} ${formResult.data.endTime}`,
                )
            ) {
                setError(
                    formResult,
                    "endTime",
                    "End time must be after start time",
                );
            } else {
                try {
                    await invoke("create_event", {
                        event: {
                            title: formResult.data.title,
                            start: `${formResult.data.startDate} ${formResult.data.startTime}`,
                            end: `${formResult.data.endDate} ${formResult.data.endTime}`,
                            location: formResult.data.location,
                            person_in_charge: formResult.data.personInCharge,
                            contact_num: formResult.data.contactNum,
                            is_approved: parseInt(approvalStatus),
                            color: color,
                        },
                    });
                    onSuccess();
                    isOpen = false;
                    toast.success("Event created!", {
                        description: `Title: ${formResult.data.title}, Start: ${formResult.data.startDate}, ${formResult.data.startTime}`,
                    });
                } catch (error) {
                    console.error(error);
                    toast("Failed to create event.");
                }
            }
        },
    });

    const { form: formData, enhance } = form;
</script>

<Dialog.Root bind:open={isOpen}>
    <Dialog.Trigger class={buttonVariants({ variant: "default" })}
        >Create New Event</Dialog.Trigger
    >
    <Dialog.Content>
        <form use:enhance>
            <Form.Field {form} name="title">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>Title</Form.Label>
                        <Input {...props} bind:value={$formData.title} />
                    {/snippet}
                </Form.Control>
                <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="startDate">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>Start Date</Form.Label>
                        <DatePicker
                            {...props}
                            bind:date={$formData.startDate}
                        />
                    {/snippet}
                </Form.Control>
                <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="startTime">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>Start Time</Form.Label>
                        <TimePicker
                            {...props}
                            bind:time={$formData.startTime}
                        />
                    {/snippet}
                </Form.Control>
            </Form.Field>
            <Form.Field {form} name="endDate">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>End Date</Form.Label>
                        <DatePicker
                            {...props}
                            bind:date={$formData.endDate}
                            minDate={$formData.startDate}
                        />
                    {/snippet}
                </Form.Control>
                <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="endTime">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>End Time</Form.Label>
                        <TimePicker {...props} bind:time={$formData.endTime} />
                    {/snippet}
                </Form.Control>
            </Form.Field>
            <Form.Field {form} name="location">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>Location</Form.Label>
                        <Input {...props} bind:value={$formData.location} />
                    {/snippet}
                </Form.Control>
                <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="personInCharge">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>Person In Charge</Form.Label>
                        <Input
                            {...props}
                            bind:value={$formData.personInCharge}
                        />
                    {/snippet}
                </Form.Control>
                <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="contactNum">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>Contact Number</Form.Label>
                        <Input {...props} bind:value={$formData.contactNum} />
                    {/snippet}
                </Form.Control>
                <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="isApproved">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>Approval Status</Form.Label>
                        <ApprovalSelect {...props} bind:approvalStatus />
                    {/snippet}
                </Form.Control>
            </Form.Field>
            <Form.Field {form} name="color">
                <Form.Control>
                    {#snippet children({ props })}
                        <Form.Label>Color</Form.Label>
                        <ColorPicker bind:value={color} />
                    {/snippet}
                </Form.Control>
            </Form.Field>
            <Dialog.Footer>
                <Dialog.Close
                    type="button"
                    class={buttonVariants({ variant: "outline" })}
                >
                    Cancel
                </Dialog.Close>
                <Button
                    type="submit"
                    class={buttonVariants({ variant: "default" })}
                    >Save changes</Button
                >
            </Dialog.Footer>
        </form>
    </Dialog.Content>
</Dialog.Root>
