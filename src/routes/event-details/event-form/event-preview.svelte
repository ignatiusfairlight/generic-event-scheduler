<script lang="ts">
    import { type Event } from "$lib/tableUtils";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import { toast } from "svelte-sonner";
    import domtoimage from "dom-to-image-more";

    const { event } = $props<{ event: Event }>();

    let isOpen = $state(false);
    let a4Ref: HTMLDivElement | null = $state(null);

    async function openDialog() {
        isOpen = true;
    }

    async function exportAsPng() {
      if (!a4Ref) return;
      try {
        const dataUrl = await domtoimage.toPng(a4Ref);
        const link = document.createElement("a");
        link.download = `${event.title.replace(/\s+/g, "_")}_event_form.png`;
        link.href = dataUrl;
        link.click();
      } catch (error) {
        console.error(error);
        toast("Failed to export as PNG.")
      }
    }
</script>

<Dialog.Root bind:open={isOpen}>
    <Dialog.Trigger
        onclick={openDialog}
        class={buttonVariants({ variant: "outline" })}>Preview</Dialog.Trigger
    >
    <Dialog.Content
        class="max-h-[90vh] overflow-y-auto max-w-fit"
        showCloseButton={false}
    >
        <div class="overflow-auto flex justify-center pt-6">
            <div
                bind:this={a4Ref}
                class="mx-auto bg-white border border-gray-300 shadow-sm"
                style="width:794px; height: 1050px; padding: 60px; box-sizing: border-box; zoom: 0.6;"
            >
                <h2
                    class="text-xl font-semibold text-center mb-8 border-b pb-4"
                >
                    Event Form
                </h2>
                <div class="space-y-4">
                    <div class="flex border-b pb-2">
                        <span class="w-40 font-medium text-sm">Title</span>
                        <span class="text-sm">{event.title}</span>
                    </div>
                    <div class="flex border-b pb-2">
                        <span class="w-40 font-medium text-sm">Start</span>
                        <span class="text-sm">{event.start}</span>
                    </div>
                    <div class="flex border-b pb-2">
                        <span class="w-40 font-medium text-sm">End</span>
                        <span class="text-sm">{event.end}</span>
                    </div>
                    <div class="flex border-b pb-2">
                        <span class="w-40 font-medium text-sm">Location</span>
                        <span class="text-sm">{event.location}</span>
                    </div>
                    <div class="flex border-b pb-2">
                        <span class="w-40 font-medium text-sm"
                            >Person In Charge</span
                        >
                        <span class="text-sm">{event.person_in_charge}</span>
                    </div>
                    <div class="flex border-b pb-2">
                        <span class="w-40 font-medium text-sm"
                            >Contact Number</span
                        >
                        <span class="text-sm">{event.contact_num}</span>
                    </div>
                    <div class="flex border-b pb-2">
                        <span class="w-40 font-medium text-sm"
                            >Approval Status</span
                        >
                        <span class="text-sm">
                            {event.is_approved === 1
                                ? "Approved"
                                : event.is_approved === 2
                                  ? "Rejected"
                                  : "Pending"}
                        </span>
                    </div>
                </div>
            </div>
        </div>
        <Dialog.Footer>
            <Button type="button" onclick={exportAsPng} class={buttonVariants({ variant: "default" })}>
                    Export as PNG
            </Button>
            <Dialog.Close
                type="button"
                class={buttonVariants({ variant: "outline" })}
            >
                Done
            </Dialog.Close>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
