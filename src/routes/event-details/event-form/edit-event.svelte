<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Form from "$lib/components/ui/form/index.js";
    import { setError, superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { type Event } from "$lib/tableUtils";
    import { formSchema } from "./schema";
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "svelte-sonner";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import { PencilLine } from "@lucide/svelte";
    import DatePicker from "./component/date-picker.svelte";
    import TimePicker from "./component/time-picker.svelte";
    import ApprovalSelect from "./component/approval-select.svelte";
    import ColorPicker from "./component/color-picker.svelte";

    const { event, onSuccess } = $props<{
        event: Event;
        onSuccess: () => void;
    }>();
    
    const [startDate, startTime] = event.start.split(" ");
    const [endDate, endTime] = event.end.split(" ");

    let approvalStatus = $state(String(event.is_approved));
    let color = $state(event.color);
    let isOpen = $state(false);

    const form = superForm(
        {
            title: event.title,
            startDate: startDate,
            startTime: startTime,
            endDate: endDate,
            endTime: endTime,
            location: event.location,
            personInCharge: event.person_in_charge,
            contactNum: event.contact_num,
            isApproved: event.is_approved,
            color: event.color,
        },
        {
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
                        await invoke("update_event", {
                            event: {
                                id: event.id,
                                title: formResult.data.title,
                                start: `${formResult.data.startDate} ${formResult.data.startTime}`,
                                end: `${formResult.data.endDate} ${formResult.data.endTime}`,
                                location: formResult.data.location,
                                person_in_charge:
                                    formResult.data.personInCharge,
                                contact_num: formResult.data.contactNum,
                                is_approved: parseInt(approvalStatus),
                                color: color,
                            },
                        });
                        onSuccess();
                        isOpen = false;
                        toast.success("Event updated!", {
                            description: `Title: ${formResult.data.title}, Start: ${formResult.data.startDate}, ${formResult.data.startTime}`,
                        });
                    } catch (error) {
                        console.error(error);
                        toast("Failed to create event.");
                    }
                }
            },
        },
    );

    const { form: formData, enhance } = form;

    async function openDialog() {
        const updatedEvent = await invoke<Event>("get_event_by_id", {
            id: event.id,
        });
        const [updatedStartDate, updatedStartTime] =
            updatedEvent.start.split(" ");
        const [updatedEndDate, updatedEndTime] = updatedEvent.end.split(" ");

        form.reset({
            data: {
                title: updatedEvent.title,
                startDate: updatedStartDate,
                startTime: updatedStartTime,
                endDate: updatedEndDate,
                endTime: updatedEndTime,
                location: updatedEvent.location,
                personInCharge: updatedEvent.person_in_charge,
                contactNum: updatedEvent.contact_num,
                isApproved: updatedEvent.is_approved,
            },
        });

        approvalStatus = String(updatedEvent.is_approved);
        color = updatedEvent.color;
        isOpen = true;
    }
</script>

<Dialog.Root bind:open={isOpen}>
    <Dialog.Trigger
        onclick={openDialog}
        class={buttonVariants({ variant: "outline" })}
        ><PencilLine /></Dialog.Trigger
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
                        <ApprovalSelect
                            {...props}
                            bind:approvalStatus
                            allowReject={true}
                        />
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
