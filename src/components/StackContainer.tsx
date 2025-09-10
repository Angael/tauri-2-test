import { Container, Stack } from "@mantine/core";
import { ComponentProps } from "react";

type Props = ComponentProps<typeof Container> & {
  children?: React.ReactNode;
};

const StackContainer = ({ children, size }: Props) => {
  return (
    <Container size={size || "md"}>
      <Stack>{children}</Stack>
    </Container>
  );
};

export default StackContainer;
