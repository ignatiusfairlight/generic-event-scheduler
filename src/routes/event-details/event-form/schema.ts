import { z } from "zod";

export const formSchema = z.object({
    title: z.string().min(2).max(50),
    startDate: z.string().min(1),
    startTime: z.string().min(1),
    endDate: z.string().min(1),
    endTime: z.string().min(1),
    location: z.string().min(5).max(25),
    personInCharge: z.string().min(5).max(25),
    contactNum: z.string().min(10).regex(/^\+?\d+$/, "Invalid phone number"),
    isApproved: z.union([z.literal(0), z.literal(1), z.literal(2)]).default(0),
    color: z.string().regex(/^#[0-9a-fA-F]{6}$/, "Invalid color").default("#3B82F6")
});

export type FormSchema = typeof formSchema;