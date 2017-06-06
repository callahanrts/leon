use super::dom_string::{DOMString};

trait EventTarget {
  // fn addEventListener(typ: DOMString, callback: Option<EventListener>, optional (AddEventListenerOptions or boolean) options);
  // fn removeEventListener(typ: DOMString, callback: Option<EventListener>, optional (EventListenerOptions or boolean) options);
  // fn dispatchEvent(Event event) -> bool;
}

// trait EventListener {
//     fn handleEvent(&mut self, event: Event);
// }
// callback interface EventListener {
//   void handleEvent(Event event);
// };

// dictionary EventListenerOptions {
//   boolean capture = false;
// };

// dictionary AddEventListenerOptions : EventListenerOptions {
//   boolean passive = false;
//   boolean once = false;
// };
